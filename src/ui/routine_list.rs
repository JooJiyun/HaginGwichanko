use iced_core::{Element, Theme};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container, row, text};

use crate::system::routine_state::RoutineInfo;
use crate::system::system_data::SystemData;
use crate::system::{RoutineChangeEvent, UIEvent};
use crate::ui::style_utils::default_container_style;

const TEXT_RUN_AT_STARTUP: &str = "run at startup";
const TEXT_NOT_RUN_AT_STARTUP: &str = "not run at startup";
const TEXT_RUN: &str = "run";
const TEXT_STOP: &str = "stop";
const TEXT_DELETE: &str = "delete";

pub fn view(
    system_data: &SystemData,
) -> iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer> {
    let mut routine_item_list = iced_widget::row![];
    for (index, routine_info) in system_data.routines.iter().enumerate() {
        routine_item_list = routine_item_list.push(routine_item_view(routine_info, index));
    }

    iced_widget::container(routine_item_list).padding(0).into()
}

fn routine_item_view(
    routine_info: &RoutineInfo,
    routine_index: usize,
) -> Element<'static, UIEvent, Theme, Renderer> {
    container(column![
        text(routine_info.name.clone()),
        row![
            text(routine_info.created_at.clone()),
            text(routine_info.last_modified.clone()),
        ],
        //
        if routine_info.run_at_startup {
            button(text(TEXT_RUN_AT_STARTUP))
                .on_press(RoutineChangeEvent::SetRunWithStartup(true).with_into(routine_index))
        } else {
            button(text(TEXT_NOT_RUN_AT_STARTUP))
                .on_press(RoutineChangeEvent::SetRunWithStartup(true).with_into(routine_index))
        },
        //
        if routine_info.is_running {
            button(text(TEXT_STOP))
                .on_press(RoutineChangeEvent::SetRunWithStartup(true).with_into(routine_index))
        } else {
            button(text(TEXT_RUN))
                .on_press(RoutineChangeEvent::SetRunWithStartup(true).with_into(routine_index))
        },
        //
        button(text(TEXT_DELETE))
            .on_press(RoutineChangeEvent::SetRunWithStartup(true).with_into(routine_index)),
    ])
    .style(default_container_style)
    .into()
}
