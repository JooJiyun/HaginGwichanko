use iced_widget::{button, column, container, row, text};

use crate::routine::method::RoutineMethod;
use crate::routine::method_button_clicker::MethodButtonClicker;
use crate::routine::runner::RoutineRunner;
use crate::system::data::AppData;
use crate::system::{UIEvent, WidgetScene};
use crate::ui::routine_common::view_routine_remote;
use crate::ui::styles::*;
use crate::ui::AppUIElement;

pub fn view_routine_list(system_data: &AppData) -> AppUIElement {
    // new buttons
    let mut new_button_list = row![].spacing(5);
    for routine_method in RoutineMethod::get_defaults() {
        let method_name = routine_method.method_name().to_string();
        new_button_list = new_button_list.push(
            button(text(method_name.clone()))
                .style(default_button_style)
                .on_press(UIEvent::CreateNewRoutine(routine_method)),
        );
    }

    // created routines
    let mut routine_item_list = column![].spacing(5);
    for (index, routine_info) in system_data.routines.iter().enumerate() {
        routine_item_list = routine_item_list.push(routine_item_view(routine_info, index));
    }

    let test_runner =
        RoutineRunner::new(RoutineMethod::ButtonClicker(MethodButtonClicker::default()));
    iced_widget::container(
        column![
            new_button_list,
            routine_item_list,
            iced_widget::slider(0.0..=100.0, 10., move |_v| {
                let mut test2 = test_runner.clone();
                test2.state_is_running = false;
                UIEvent::UpdateRoutine(0, test2)
            },)
        ]
        .spacing(5),
    )
    .style(default_container_style)
    .padding(5)
    .into()
}

fn routine_item_view(routine_info: &RoutineRunner, routine_index: usize) -> AppUIElement {
    container(row![
        button(column![
            text(routine_info.routine_name.clone()),
            row![
                text(routine_info.time_created_at.clone()),
                text(routine_info.time_last_modified.clone()),
            ]
        ])
        .style(default_button_style)
        .on_press(WidgetScene::RoutineDetail(routine_index).into()),
        view_routine_remote(routine_info, routine_index),
    ])
    .style(default_container_style)
    .into()
}
