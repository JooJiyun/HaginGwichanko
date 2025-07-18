use iced_core::{Element, Theme};
use iced_wgpu::Renderer;
use iced_widget::{button, column, container, row, text};

use crate::system::data::SystemData;
use crate::system::routine::RoutineInfo;
use crate::system::{RoutineChangeEvent, UIEvent, WidgetScene};
use crate::ui::const_text::*;
use crate::ui::styles::*;

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
        button(text(TEXT_DETAIL))
            .style(default_button_style)
            .on_press(WidgetScene::RoutineDetail(routine_index).into()),
        //
        if routine_info.run_at_startup {
            button(text(TEXT_RUN_AT_STARTUP))
                .style(green_button_style)
                .on_press(RoutineChangeEvent::SetRunWithStartup(true).with_into(routine_index))
        } else {
            button(text(TEXT_NOT_RUN_AT_STARTUP))
                .style(gray_button_style)
                .on_press(RoutineChangeEvent::SetRunWithStartup(false).with_into(routine_index))
        },
        //
        if routine_info.is_running {
            button(text(TEXT_STOP))
                .style(red_button_style)
                .on_press(RoutineChangeEvent::Stop.with_into(routine_index))
        } else {
            button(text(TEXT_RUN))
                .style(green_button_style)
                .on_press(RoutineChangeEvent::Run.with_into(routine_index))
        },
        //
        button(text(TEXT_DELETE))
            .style(red_button_style)
            .on_press(RoutineChangeEvent::Delete.with_into(routine_index)),
    ])
    .style(default_container_style)
    .into()
}
