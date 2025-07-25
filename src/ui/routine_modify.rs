use iced_core::Length::{self, Fill};
use iced_core::{Color, Element, Padding, Theme};
use iced_wgpu::Renderer;
use iced_widget::{button, column, row, text, text_input};
use iced_widget::{container, slider};

use crate::routine::method::RoutineMethod;
use crate::routine::method_button_clicker::MethodButtonClicker;
use crate::routine::method_position_clicker::MethodPositionClicker;
use crate::routine::runner::RoutineRunner;
use crate::system::data::AppData;
use crate::system::outview::{RootViewElement, ViewElementNode};
use crate::system::{UIEvent, WidgetScene};
use crate::ui::const_text::*;
use crate::ui::styles::*;
use crate::ui::AppUIElement;

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
) -> AppUIElement {
    let Some(modify_routine) = &system_data.tmp_modify_routine else {
        return row![].into();
    };

    container(column![
        // back button
        button(text(TEXT_CANCEL))
            .style(default_button_style)
            .on_press(if is_new_routine {
                UIEvent::CancelCreateRoutine
            } else {
                WidgetScene::RoutineList.into()
            }),
        // routine name, done button
        container(row![
            text(modify_routine.routine_name.clone()),
            button(text(TEXT_DONE))
                .style(green_button_style)
                .on_press(UIEvent::UpdateRoutine(
                    routine_index,
                    modify_routine.clone()
                )),
        ]),
        // routine common
        view_edit_routine_method_common(modify_routine.clone()),
        // routine method
        match &modify_routine.routine_method {
            RoutineMethod::PositiionClicker(position_clicker) => {
                view_edit_position_clicker(&position_clicker)
            }
            RoutineMethod::ButtonClicker(button_clicker) => {
                view_edit_button_clicker(&button_clicker, system_data)
            }
        },
        text_input("Type something here...", "default")
            .on_input(move |v| { UIEvent::ChangeRoutineRunState(routine_index, v.len() > 10) })
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn view_edit_routine_method_common(routine_info: RoutineRunner) -> AppUIElement {
    container(slider(0.0..=100.0, 50.0, move |v| {
        let mut tmp_routine_info = routine_info.clone();
        tmp_routine_info.loop_interval = v as u64;
        UIEvent::ModifyTempRoutine(tmp_routine_info)
    }))
    .width(Length::Fill)
    .into()
}

fn view_edit_position_clicker(click_position_info: &MethodPositionClicker) -> AppUIElement {
    iced_widget::container(row![
        text(click_position_info.mouse_speed.to_second_string()),
        text(format!(
            "{}, {}",
            click_position_info.target_position.0, click_position_info.target_position.1
        )),
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn view_edit_button_clicker(
    click_button_if_find_info: &MethodButtonClicker,
    system_data: &AppData,
) -> AppUIElement {
    let filter_tree = view_routine_filter(system_data);

    let mut filter_view = column![];
    for (index, filter) in click_button_if_find_info.filter_list.iter().enumerate() {
        let mut filter_inner_view = row![
            text(filter.text.to_string()),
            text(filter.depth.to_string())
        ];
        match filter.index {
            Some(index) => filter_inner_view = filter_inner_view.push(text(index.to_string())),
            None => {
                if index == click_button_if_find_info.filter_list.len() - 1 {
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
        text(click_button_if_find_info.mouse_speed.to_second_string()),
        filter_view,
        text(click_button_if_find_info.find_time_limit.to_second_string())
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

/* #region -- OutView Tree -- */
pub fn view_routine_filter(system_data: &crate::system::data::AppData) -> AppUIElement {
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
    .padding(10)
    .style(default_container_style)
    .into()
}

fn view_element_recursive(element_node: &ViewElementNode) -> AppUIElement {
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

/* #endregion */
