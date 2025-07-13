use iced_core::{color, Theme};

pub fn transparent_container(_theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        text_color: Some(color!(0x564578)),
        background: Some(iced_core::Background::Color(color!(0x375786))),
        ..Default::default()
    }
}
