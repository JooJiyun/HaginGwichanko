use std::sync::{Arc, Mutex};

use iced_wgpu::Renderer;
use iced_winit::core::{Element, Theme};
use iced_winit::runtime::{Program, Task};

use crate::system::system_data::SystemData;
use crate::system::UIEvent;
use crate::ui::root::view;

pub struct UIContext {
    pub messages: Vec<UIEvent>,
    pub system_data: Arc<Mutex<SystemData>>,
}

impl UIContext {
    pub fn new(data: Arc<Mutex<SystemData>>) -> UIContext {
        UIContext {
            messages: vec![],
            system_data: data,
        }
    }
}

impl Program for UIContext {
    type Theme = Theme;
    type Message = UIEvent;
    type Renderer = Renderer;

    fn update(&mut self, message: UIEvent) -> Task<UIEvent> {
        match message {
            // UIEvent::ButtonPressed => {
            //     let mut data_value = self.system_data.lock().expect("failed get arc mutex");
            //     data_value.count += 1;
            //     self.messages.push(message.clone());
            // }
            _ => {}
        }

        Task::none()
    }

    fn view(&self) -> Element<UIEvent, Theme, Renderer> {
        let data_value = self.system_data.lock().expect("failed get arc mutex");
        let system_data = data_value.clone();
        view(&system_data).into()
    }
}
