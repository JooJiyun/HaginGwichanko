use iced_core::{color, Color, Element, Padding, Theme};
use iced_wgpu::Renderer;
use iced_widget::{column, container, text};

use crate::system::view_func::ViewElementNode;
use crate::system::UIEvent;

pub fn view(view_element_node: &ViewElementNode) -> Element<'static, UIEvent, Theme, Renderer> {
    container(column![text(view_element_node.info.name.clone()).color(Color::BLACK),].spacing(10))
        .padding(0)
        .style(view_tree_item_container_style)
        .into()
}

fn view_tree_item_container_style(_theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        text_color: Some(color!(0x564578)),
        background: Some(iced_core::Background::Color(color!(0x375786))),
        ..Default::default()
    }
}
