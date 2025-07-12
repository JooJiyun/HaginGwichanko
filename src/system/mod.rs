use std::{process, sync::mpsc::Sender};

use uiautomation::dialogs::show_message;
use winit::application::ApplicationHandler;

use crate::system::system_tray::SystemTrayHandle;

pub mod single_instance;
mod system_tray;

pub enum SystemEvent {
    SystemTrayEvent(tray_icon::menu::MenuEvent),
}

pub struct TerminateThread();

pub struct System {
    system_tray_handle: system_tray::SystemTrayHandle,

    routine_senders: Vec<Sender<TerminateThread>>,
}

impl Default for System {
    fn default() -> Self {
        System {
            system_tray_handle: SystemTrayHandle::default(),

            routine_senders: vec![],
        }
    }
}

impl ApplicationHandler<SystemEvent> for System {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        if winit::event::StartCause::Init == cause {
            if let Err((msg, err)) = self.system_tray_handle.init() {
                show_message(&msg, "Error");
                eprintln!("{}", err);
                process::exit(1);
            }
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: SystemEvent) {
        match event {
            SystemEvent::SystemTrayEvent(menu_event) => {
                match self.system_tray_handle.parse_event(menu_event) {
                    system_tray::SystemTrayEvent::Open => self.open(),
                    system_tray::SystemTrayEvent::Quit => self.quit(),
                    system_tray::SystemTrayEvent::Start => self.start(),
                    system_tray::SystemTrayEvent::Stop => self.stop(),
                    system_tray::SystemTrayEvent::Invalid => show_message("", "Warning"),
                }
            }
        }
    }
}

impl System {
    fn open(&mut self) {}

    fn quit(&mut self) {
        // thread들 모두 종료
        for sender in &self.routine_senders {
            if let Err(e) = sender.send(TerminateThread()) {
                eprintln!("failed send terminate event : {:?}", e);
            }
        }
        std::process::exit(0);
    }

    fn start(&mut self) {}

    fn stop(&mut self) {}
}
