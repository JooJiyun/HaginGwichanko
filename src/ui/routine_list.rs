use iced_core::{Element, Theme};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container, row, text};

use crate::system::data::AppData;
use crate::system::routine::{RoutineInfo, RoutineMethod};
use crate::system::{RoutineChangeEvent, UIEvent, WidgetScene};
use crate::ui::const_text::*;
use crate::ui::styles::*;

pub fn view_routine_list(
    system_data: &AppData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
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

    iced_widget::container(column![new_button_list, routine_item_list].spacing(5))
        .style(default_container_style)
        .padding(5)
        .into()
}

fn routine_item_view(
    routine_info: &RoutineInfo,
    routine_index: usize,
) -> Element<'static, UIEvent, Theme, Renderer> {
    container(row![
        button(column![
            text(routine_info.name.clone()),
            row![
                text(routine_info.created_at.clone()),
                text(routine_info.last_modified.clone()),
            ]
        ])
        .style(default_button_style)
        .on_press(WidgetScene::RoutineDetail(routine_index).into()),
        row![
            // run at startup
            if routine_info.run_at_startup {
                button(text(TEXT_RUN_AT_STARTUP))
                    .style(green_button_style)
                    .on_press(RoutineChangeEvent::SetRunAtStartup(true).with_into(routine_index))
            } else {
                button(text(TEXT_NOT_RUN_AT_STARTUP))
                    .style(gray_button_style)
                    .on_press(RoutineChangeEvent::SetRunAtStartup(false).with_into(routine_index))
            },
            // is running
            if routine_info.is_running {
                button(text(TEXT_STOP))
                    .style(red_button_style)
                    .on_press(RoutineChangeEvent::ChangeRunState(false).with_into(routine_index))
            } else {
                button(text(TEXT_RUN))
                    .style(green_button_style)
                    .on_press(RoutineChangeEvent::ChangeRunState(true).with_into(routine_index))
            },
            // delete
            button(text(TEXT_DELETE))
                .style(red_button_style)
                .on_press(RoutineChangeEvent::Delete.with_into(routine_index)),
        ]
    ])
    .style(default_container_style)
    .into()
}
