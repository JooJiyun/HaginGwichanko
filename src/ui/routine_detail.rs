use iced_core::Length;
use iced_widget::{button, column, container, row, scrollable, text};

use crate::routine::method::RoutineMethod;
use crate::routine::method_button_clicker::MethodButtonClicker;
use crate::routine::method_position_clicker::MethodPositionClicker;
use crate::routine::runner::RoutineRunner;
use crate::system::data::AppData;
use crate::system::WidgetScene;
use crate::ui::routine_common::view_routine_remote;
use crate::ui::styles::{default_button_style, default_container_style};
use crate::ui::{const_text::*, AppUIElement};

pub fn view_routine_detail(system_data: &AppData, routine_index: usize) -> AppUIElement {
    let selected_routine = &system_data.routines[routine_index];

    container(column![
        // back button
        button(text(TEXT_BACK))
            .style(default_button_style)
            .on_press(WidgetScene::RoutineList.into()),
        // routine name, remote
        container(row![
            text(selected_routine.routine_name.clone()),
            row![view_routine_remote(selected_routine, routine_index)],
        ]),
        // modify button
        button(TEXT_MODIFY)
            .style(default_button_style)
            .on_press(WidgetScene::RoutineModify(routine_index, false).into()),
        // routine common
        view_routine_method_common(selected_routine),
        // routine method
        match &selected_routine.routine_method {
            RoutineMethod::PositiionClicker(click_position_info) => {
                view_routine_method_click_position(&click_position_info)
            }
            RoutineMethod::ButtonClicker(click_button_if_find_info) => {
                view_routine_method_click_button(&click_button_if_find_info)
            }
        }
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn view_routine_method_common(routine_info: &RoutineRunner) -> AppUIElement {
    container(row![view_routine_method_element_form(
        "",
        &routine_info.loop_interval.to_string()
    ),])
    .width(Length::Fill)
    .into()
}

fn view_routine_method_click_position(click_position_info: &MethodPositionClicker) -> AppUIElement {
    container(row![
        view_routine_method_element_form(
            "",
            &click_position_info
                .mouse_speed
                .value
                .as_millis()
                .to_string()
        ),
        view_routine_method_element_form(
            "",
            &format!(
                "{}, {}",
                click_position_info.target_position.0, click_position_info.target_position.1
            )
            .to_string()
        ),
    ])
    .style(default_container_style)
    .padding(0)
    .into()
}

fn view_routine_method_element_form(tag: &str, value: &str) -> AppUIElement {
    container(row![
        text(tag.to_string()),
        row![].width(Length::Fill),
        text(value.to_string()),
    ])
    .width(Length::Fill)
    .into()
}

fn view_routine_method_click_button(
    click_button_if_find_info: &MethodButtonClicker,
) -> AppUIElement {
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

    container(scrollable(row![
        text(
            click_button_if_find_info
                .mouse_speed
                .value
                .as_millis()
                .to_string()
        ),
        filter_view,
        text(
            click_button_if_find_info
                .find_time_limit
                .value
                .as_millis()
                .to_string()
        )
    ]))
    .style(default_container_style)
    .padding(0)
    .into()
}
