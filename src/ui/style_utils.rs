use iced_core::{color, Border, Theme};

pub fn default_container_style(_theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        text_color: Some(color!(0x000000)),
        background: Some(iced_core::Background::Color(color!(0xf5d9d7))),
        border: Border {
            color: color!(0x454040),
            width: 2.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}
