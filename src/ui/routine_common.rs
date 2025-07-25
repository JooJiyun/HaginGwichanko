use iced_widget::{button, row, text};

use crate::routine::runner::RoutineRunner;
use crate::system::UIEvent;
use crate::ui::const_text::*;
use crate::ui::styles::*;
use crate::ui::AppUIElement;

pub fn view_routine_remote(routine_info: &RoutineRunner, routine_index: usize) -> AppUIElement {
    row![
        // state run at startup
        if routine_info.state_run_at_startup {
            button(text(TEXT_NOT_RUN_AT_STARTUP))
                .style(gray_button_style)
                .on_press(UIEvent::ChangeRoutineRunAtStartUpState(
                    routine_index,
                    false,
                ))
        } else {
            button(text(TEXT_RUN_AT_STARTUP))
                .style(green_button_style)
                .on_press(UIEvent::ChangeRoutineRunAtStartUpState(routine_index, true))
        },
        // run or stop
        if routine_info.state_is_running {
            button(text(TEXT_STOP))
                .style(red_button_style)
                .on_press(UIEvent::ChangeRoutineRunState(routine_index, false))
        } else {
            button(text(TEXT_RUN))
                .style(green_button_style)
                .on_press(UIEvent::ChangeRoutineRunState(routine_index, true))
        },
        // delete
        button(text(TEXT_DELETE))
            .style(red_button_style)
            .on_press(UIEvent::DeleteRoutine(routine_index))
    ]
    .into()
}
