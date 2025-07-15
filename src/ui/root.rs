use iced_wgpu::Renderer;
use iced_widget::{button, column, container, text};
use iced_winit::core::{Color, Element, Length::*, Theme};

use crate::system::data::SystemData;
use crate::system::UIEvent;
use crate::ui::style_utils::transparent_container;

pub fn view(system_data: &SystemData) -> Element<'static, UIEvent, Theme, Renderer> {
    container(
        column![
            text("Background color").color(Color::WHITE),
            button("content").on_press(UIEvent::ButtonPressed)
        ]
        .spacing(10),
    )
    .padding(10)
    .align_bottom(Fill)
    .style(transparent_container)
    .into()
}
