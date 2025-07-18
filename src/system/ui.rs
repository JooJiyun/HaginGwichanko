use std::sync::{Arc, Mutex};

use iced_wgpu::Renderer;
use iced_winit::core::{Element, Theme};
use iced_winit::runtime::{Program, Task};

use crate::system::data::SystemData;
use crate::system::view::get_processes_root_node;
use crate::system::UIEvent;
use crate::ui::root::view;

pub struct APPUI {
    pub messages: Vec<UIEvent>,
    pub system_data: Arc<Mutex<SystemData>>,
}

impl APPUI {
    pub fn new(data: Arc<Mutex<SystemData>>) -> APPUI {
        APPUI {
            messages: vec![],
            system_data: data,
        }
    }
}

impl Program for APPUI {
    type Theme = Theme;
    type Message = UIEvent;
    type Renderer = Renderer;

    fn update(&mut self, message: UIEvent) -> Task<UIEvent> {
        match message {
            UIEvent::OpenWidgetScene(widget_scene) => {
                let mut data_value = self.system_data.lock().expect("failed get arc mutex");
                {
                    data_value.current_widget_scene = widget_scene;
                }
            }
            UIEvent::RoutineChanged(routine_change_event, _) => match routine_change_event {
                crate::system::RoutineChangeEvent::Rename(_) => todo!(),
                crate::system::RoutineChangeEvent::Run => todo!(),
                crate::system::RoutineChangeEvent::Stop => todo!(),
                crate::system::RoutineChangeEvent::Delete => todo!(),
                crate::system::RoutineChangeEvent::SetRunWithStartup(_) => todo!(),
            },
            UIEvent::UpdateViewState => {
                let mut data_value = self.system_data.lock().expect("failed get arc mutex");
                {
                    // [!todo]
                    data_value.view_tree_processes = get_processes_root_node().unwrap_or(vec![]);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<UIEvent, Theme, Renderer> {
        let data_value = self.system_data.lock().expect("failed get arc mutex");
        let system_data = data_value.clone();
        view(&system_data).into()
    }
}
