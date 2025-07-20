use iced_core::Length::Fill;
use iced_core::{Color, Element, Padding, Theme};
use iced_wgpu::Renderer;
use iced_widget::container;
use iced_widget::{button, column, row, text};

use crate::system::data::AppData;
use crate::system::outview::{RootViewElement, ViewElementNode};
use crate::system::routine::{ClickButtonIfFindInfo, ClickPositionInfo, RoutineMethod};
use crate::system::UIEvent;
use crate::ui::const_text::*;
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

pub fn view_routine_modify(
    system_data: &AppData,
    routine_index: usize,
    is_new_routine: bool,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let selected_routine = &system_data.routines[routine_index];

    iced_widget::container(column![
        row![button(text(TEXT_DONE)), button(text(TEXT_BACK)),],
        text(selected_routine.name.clone()),
        routine_method_view(&selected_routine.routin_method, system_data)
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn routine_method_view(
    routine_method: &RoutineMethod,
    system_data: &AppData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    iced_widget::container(match routine_method {
        RoutineMethod::None => row![].into(),
        RoutineMethod::ClickPosition(click_position_info) => {
            routine_click_position_view(click_position_info)
        }
        RoutineMethod::ClickButtonIfFind(click_button_if_find_info) => {
            routine_click_button_if_find_view(click_button_if_find_info, system_data)
        }
    })
    .style(default_container_style)
    .padding(0)
    .into()
}

fn routine_click_position_view(
    click_position_info: &ClickPositionInfo,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    iced_widget::container(row![
        text(click_position_info.mouse_speed),
        text(format!(
            "{}, {}",
            click_position_info.target_position.0, click_position_info.target_position.1
        )),
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn routine_click_button_if_find_view(
    click_button_if_find_info: &ClickButtonIfFindInfo,
    system_data: &AppData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let filter_tree = view_routine_filter(system_data);

    let mut filter_view = column![];
    for (index, filter) in click_button_if_find_info.filter.iter().enumerate() {
        let mut filter_inner_view = row![
            text(filter.text.to_string()),
            text(filter.depth.to_string())
        ];
        match filter.index {
            Some(index) => filter_inner_view = filter_inner_view.push(text(index.to_string())),
            None => {
                if index == click_button_if_find_info.filter.len() - 1 {
                    filter_inner_view = filter_inner_view.push(text("first"));
                } else {
                    filter_inner_view = filter_inner_view.push(text("all"));
                }
            }
        }
        filter_view = filter_view.push(filter_inner_view);
    }
    iced_widget::container(row![
        button(text("refresh")),
        filter_tree,
        text(click_button_if_find_info.mouse_speed.to_string()),
        filter_view,
        text(click_button_if_find_info.find_time_limit.to_string())
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

/* #region -- OutView Tree -- */
pub fn view_routine_filter(
    system_data: &crate::system::data::AppData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let mut view_trees = iced_widget::row![];
    for process_root_view_element in &system_data.outview_trees {
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
