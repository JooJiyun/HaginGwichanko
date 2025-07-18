use iced_widget::{button, column, row, text};

use crate::system::data::SystemData;
use crate::system::routine::{ClickButtonIfFindInfo, ClickPositionInfo, RoutineMethod};
use crate::ui::styles::default_container_style;
use crate::ui::{const_text::*, routine_filter};

pub fn view(
    system_data: &SystemData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let selected_routine = &system_data.routines[system_data.selected_routine_index];

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
    system_data: &SystemData,
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
    system_data: &SystemData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let filter_tree = routine_filter::view(system_data);

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
