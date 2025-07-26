use std::process;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::keyboard::ModifiersState;

use uiautomation::dialogs::show_message;

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Engine, Renderer};

use iced_winit::conversion;
use iced_winit::core::mouse;
use iced_winit::core::renderer;
use iced_winit::core::{Color, Font, Pixels, Size, Theme};
use iced_winit::futures;
use iced_winit::runtime::program;
use iced_winit::runtime::Debug;
use iced_winit::winit;
use iced_winit::Clipboard;

use crate::show_error_with_terminate;
use crate::system::data::AppData;
use crate::system::tray::{self, SystemTrayHandle};
use crate::system::{ui, AppEvent, TerminateThreadEvent, WidgetScene};

pub struct App {
    system_tray_handle: tray::SystemTrayHandle,
    visual_state: VisualState,
    routine_senders: Vec<(usize, Sender<TerminateThreadEvent>)>,
    data: Arc<Mutex<AppData>>,
}

enum VisualState {
    Hidden,
    Shown {
        window: Arc<winit::window::Window>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        format: wgpu::TextureFormat,
        engine: Engine,
        renderer: Renderer,
        state: program::State<ui::AppUI>,
        cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
        clipboard: Clipboard,
        viewport: Viewport,
        modifiers: ModifiersState,
        resized: bool,
        debug: Debug,
    },
}

impl Default for App {
    fn default() -> Self {
        App {
            system_tray_handle: SystemTrayHandle::default(),
            visual_state: VisualState::Hidden,
            routine_senders: vec![],
            data: Arc::new(Mutex::new(AppData::default())),
        }
    }
}

impl ApplicationHandler<AppEvent> for App {
    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            self.visual_state = VisualState::Hidden
        }

        let VisualState::Shown {
            window,
            device,
            queue,
            surface,
            format,
            engine,
            renderer,
            state,
            cursor_position,
            clipboard,
            viewport,
            modifiers,
            resized,
            debug,
        } = &mut self.visual_state
        else {
            return;
        };

        match event {
            WindowEvent::RedrawRequested => {
                if *resized {
                    let size = window.inner_size();

                    *viewport = Viewport::with_physical_size(
                        Size::new(size.width, size.height),
                        window.scale_factor(),
                    );

                    surface.configure(
                        device,
                        &wgpu::SurfaceConfiguration {
                            format: *format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            width: size.width,
                            height: size.height,
                            present_mode: wgpu::PresentMode::AutoVsync,
                            alpha_mode: wgpu::CompositeAlphaMode::Auto,
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        },
                    );

                    *resized = false;
                }

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        // And then iced on top
                        renderer.present(
                            engine,
                            device,
                            queue,
                            &mut encoder,
                            None,
                            frame.texture.format(),
                            &view,
                            viewport,
                            &debug.overlay(),
                        );

                        // Then we submit the work
                        engine.submit(queue, encoder);
                        frame.present();

                        // Update the mouse cursor
                        window.set_cursor(iced_winit::conversion::mouse_interaction(
                            state.mouse_interaction(),
                        ));
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            show_error_with_terminate("out of memory", "render memory out");
                        }
                        _ => {
                            // Try rendering again next frame.
                            window.request_redraw();
                        }
                    },
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                *cursor_position = Some(position);
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                *modifiers = new_modifiers.state();
            }
            WindowEvent::Resized(_) => {
                *resized = true;
            }
            _ => {}
        }

        // windows event -> iced에서 받을 수 있도록 변환
        if let Some(event) =
            iced_winit::conversion::window_event(event, window.scale_factor(), *modifiers)
        {
            state.queue_event(event);
        }

        // pending 된 이벤트 처리
        if !state.is_queue_empty() {
            // ui(iced) 업데이트
            let (_, _maybe_task) = state.update(
                viewport.logical_size(),
                cursor_position
                    .map(|p| conversion::cursor_position(p, viewport.scale_factor()))
                    .map(mouse::Cursor::Available)
                    .unwrap_or(mouse::Cursor::Unavailable),
                renderer,
                &Theme::Dark,
                &renderer::Style {
                    text_color: Color::BLACK,
                },
                clipboard,
                debug,
            );

            // running state가 바뀐 routine 처리
            {
                // let data = self.data.lock().expect("main lock");
                // for routine_runner in &data.routines {}
            }

            // 다시그리기
            window.request_redraw();
        }
    }

    fn new_events(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if winit::event::StartCause::Init == cause {
            if let Err((msg, err)) = self.system_tray_handle.init() {
                eprintln!("{}", err);
                show_message(&msg, "Error");
                process::exit(1);
            }
            self.open_window(event_loop);
        }
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::SystemTrayMenuEvent(menu_event) => {
                let system_tray_event = self.system_tray_handle.parse_menu_event(menu_event);
                self.handle_tray_event(system_tray_event, event_loop);
            }
            AppEvent::SystemTrayIconEvent(icon_event) => {
                let system_tray_event = self.system_tray_handle.parse_icon_event(icon_event);
                self.handle_tray_event(system_tray_event, event_loop);
            }
        }
    }

    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}
}

impl App {
    fn handle_tray_event(
        &mut self,
        tray_event: tray::SystemTrayEvent,
        event_loop: &winit::event_loop::ActiveEventLoop,
    ) {
        match tray_event {
            tray::SystemTrayEvent::Open => self.open_window(event_loop),
            tray::SystemTrayEvent::Quit => self.quit_process(event_loop),
            tray::SystemTrayEvent::Invalid => {}
        }
    }

    fn open_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // 이미 윈도우가 띄워져 있는 경우는 패스
        if let VisualState::Shown { .. } = self.visual_state {
            return;
        }

        // 새 윈도우의 시작화면은 routine list
        {
            let mut data_value = self.data.lock().expect("main lock");
            data_value.current_widget_scene = WidgetScene::RoutineList;
        }

        let mut window_setting = winit::window::WindowAttributes::default();
        window_setting.transparent = true;

        let window = Arc::new(
            event_loop
                .create_window(window_setting)
                .expect("Create window"),
        );

        let physical_size = window.inner_size();
        let viewport = Viewport::with_physical_size(
            Size::new(physical_size.width, physical_size.height),
            window.scale_factor(),
        );
        let clipboard = Clipboard::connect(window.clone());
        let backend = wgpu::util::backend_bits_from_env().unwrap_or_default();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: backend,
            ..Default::default()
        });
        let surface = instance
            .create_surface(window.clone())
            .expect("Create window surface");

        let (format, adapter, device, queue) = futures::futures::executor::block_on(async {
            let adapter =
                wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
                    .await
                    .expect("Create adapter");

            let adapter_features = adapter.features();

            let capabilities = surface.get_capabilities(&adapter);

            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: None,
                        required_features: adapter_features & wgpu::Features::default(),
                        required_limits: wgpu::Limits::default(),
                    },
                    None,
                )
                .await
                .expect("Request device");

            (
                capabilities
                    .formats
                    .iter()
                    .copied()
                    .find(wgpu::TextureFormat::is_srgb)
                    .or_else(|| capabilities.formats.first().copied())
                    .expect("Get preferred format"),
                adapter,
                device,
                queue,
            )
        });

        surface.configure(
            &device,
            &wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format,
                width: physical_size.width,
                height: physical_size.height,
                present_mode: wgpu::PresentMode::AutoNoVsync,
                alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            },
        );

        // Initialize scene and GUI controls
        let controls = ui::AppUI::new(self.data.clone());

        // Initialize iced
        let mut debug = Debug::new();
        let engine = Engine::new(&adapter, &device, &queue, format, None);
        let mut renderer = Renderer::new(&device, &engine, Font::default(), Pixels::from(16));

        let state =
            program::State::new(controls, viewport.logical_size(), &mut renderer, &mut debug);

        // You should change this if you want to render continuously
        event_loop.set_control_flow(ControlFlow::Wait);

        self.visual_state = VisualState::Shown {
            window,
            device,
            queue,
            surface,
            format,
            engine,
            renderer,
            state,
            cursor_position: None,
            modifiers: ModifiersState::default(),
            clipboard,
            viewport,
            resized: false,
            debug,
        };
    }

    fn quit_process(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // thread들 모두 종료
        for (_thread_id, sender) in &self.routine_senders {
            if let Err(e) = sender.send(TerminateThreadEvent) {
                eprintln!("failed send terminate event : {:?}", e);
            }
        }

        // event loop와 process 모두 종료
        event_loop.exit();
        std::process::exit(0);
    }
}
