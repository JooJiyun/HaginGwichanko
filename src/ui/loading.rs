use iced_widget::{container, row};

use crate::ui::AppUIElement;

pub fn view_loading() -> AppUIElement {
    container(row![]).padding(0).into()
}
