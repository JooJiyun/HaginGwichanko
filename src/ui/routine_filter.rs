use iced_core::Length::Fill;
use iced_core::{Color, Element, Padding, Theme};
use iced_wgpu::Renderer;
use iced_widget::{column, container, row, text};

use crate::system::view::{RootViewElement, ViewElementNode};
use crate::system::UIEvent;
use crate::ui::styles::default_container_style;

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
    system_data: &crate::system::data::SystemData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let mut view_trees = iced_widget::row![];
    for process_root_view_element in &system_data.view_tree_processes {
        view_trees = view_trees.push(filter_tree_view(process_root_view_element));
    }

    iced_widget::container(view_trees).padding(0).into()
}

fn filter_tree_view(
    process_root_view_element: &RootViewElement,
) -> Element<'static, UIEvent, Theme, Renderer> {
    container(column![
        text(process_root_view_element.process_name.clone()).color(Color::BLACK),
        view_element_recursive(&process_root_view_element.root_node)
    ])
    .padding(TREE_ITEM_ROOT_PADDING)
    .style(default_container_style)
    .into()
}

fn view_element_recursive(
    element_node: &ViewElementNode,
) -> Element<'static, UIEvent, Theme, Renderer> {
    let mut element_row = row![];

    // push self
    element_row = element_row.push(
        container(column![text(element_node.info.name.clone()).color(Color::BLACK),].spacing(10))
            .padding(0)
            .style(default_container_style),
    );

    // push childs
    for child_element_node in &element_node.childs {
        element_row = element_row.push(view_element_recursive(child_element_node));
    }

    container(element_row)
        .padding(TREE_ITEM_DEPTH_PADDING)
        .align_bottom(Fill)
        .into()
}
