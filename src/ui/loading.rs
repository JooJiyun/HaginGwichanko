use iced_widget::{container, row, text};

use crate::ui::AppUIElement;

pub fn view_loading() -> AppUIElement {
    container(row![text("loading...")]).padding(0).into()
}
