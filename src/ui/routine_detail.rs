use iced_widget::{button, column, container, row, text};

use crate::system::data::AppData;
use crate::system::routine::{ClickButtonIfFindInfo, ClickPositionInfo, RoutineMethod};
use crate::ui::styles::default_container_style;
use crate::ui::{const_text::*, AppUIElement};

pub fn view_routine_detail(system_data: &AppData, routine_index: usize) -> AppUIElement {
    let selected_routine = &system_data.routines[routine_index];

    container(column![
        row![button(text(TEXT_MODIFY)), button(text(TEXT_BACK)),],
        text(selected_routine.name.clone()),
        view_routine_element(&selected_routine.routin_method)
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn view_routine_element(routine_method: &RoutineMethod) -> AppUIElement {
    container(match routine_method {
        RoutineMethod::None => row![].into(),
        RoutineMethod::ClickPosition(click_position_info) => {
            routine_click_position_view(click_position_info)
        }
        RoutineMethod::ClickButtonIfFind(click_button_if_find_info) => {
            routine_click_button_if_find_view(click_button_if_find_info)
        }
    })
    .style(default_container_style)
    .padding(0)
    .into()
}

fn routine_click_position_view(click_position_info: &ClickPositionInfo) -> AppUIElement {
    container(row![
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
) -> AppUIElement {
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
    container(row![
        text(click_button_if_find_info.mouse_speed.to_string()),
        filter_view,
        text(click_button_if_find_info.find_time_limit.to_string())
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}
