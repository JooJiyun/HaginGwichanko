use iced_core::Length::Fill;
use iced_core::{color, Color, Element, Padding, Theme};
use iced_wgpu::Renderer;
use iced_widget::{column, container, row, text};

use crate::system::view_func::{ProcessRootViewElement, ViewElementNode};
use crate::system::UIEvent;

const TREE_ITEM_ROOT_PADDING: Padding = Padding {
    top: 10.,
    right: 10.,
    bottom: 10.,
    left: 10.,
};

const TREE_ITEM_DEPTH_PADDING: Padding = Padding {
    top: 0.,
    right: 0.,
    bottom: 0.,
    left: 10.,
};

pub fn view(
    process_root_view_element: &ProcessRootViewElement,
) -> Element<'static, UIEvent, Theme, Renderer> {
    container(column![
        text(process_root_view_element.process_name.clone()).color(Color::BLACK),
        view_element_recursive(&process_root_view_element.root_node)
    ])
    .padding(TREE_ITEM_ROOT_PADDING)
    .style(view_tree_container_style)
    .into()
}

fn view_element_recursive(
    element_node: &ViewElementNode,
) -> Element<'static, UIEvent, Theme, Renderer> {
    let mut element_row = row![];
    for child_element_node in &element_node.childs {
        element_row = element_row.push(view_element_recursive(child_element_node));
    }

    container(element_row)
        .padding(TREE_ITEM_DEPTH_PADDING)
        .align_bottom(Fill)
        .style(view_tree_container_style)
        .into()
}

fn view_tree_container_style(_theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        text_color: Some(color!(0x564578)),
        background: Some(iced_core::Background::Color(color!(0x375786))),
        ..Default::default()
    }
}
