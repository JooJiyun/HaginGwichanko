use iced_core::border::Radius;
use iced_core::{color, Background, Border, Color, Shadow, Theme};
use iced_widget::button::Status;

pub fn default_container_style(_theme: &Theme) -> iced_widget::container::Style {
    iced_widget::container::Style {
        text_color: Some(color!(0x505050)),
        background: Some(Background::Color(color!(0xffffff))),
        border: Border {
            color: color!(0x505050),
            width: 2.0,
            radius: 2.0.into(),
        },
        ..Default::default()
    }
}

pub fn default_button_style(_theme: &Theme, status: Status) -> iced_widget::button::Style {
    iced_widget::button::Style {
        background: Some(Background::Color(color!(0xffffff))),
        text_color: color!(0x505050),
        border: Border {
            color: match status {
                Status::Active => color!(0x787878),
                Status::Disabled => color!(0x505050),
                Status::Hovered => color!(0x505050),
                Status::Pressed => color!(0x505050),
            },
            width: match status {
                Status::Active => 1.0,
                Status::Disabled => 0.0,
                Status::Hovered => 2.0,
                Status::Pressed => 2.0,
            },
            radius: Radius::new(4),
        },
        shadow: Shadow::default(),
    }
}

pub fn red_button_style(_theme: &Theme, status: Status) -> iced_widget::button::Style {
    button_style_inner(status, color!(0xf27979), color!(0xf03c3c), color!(0x942e2e))
}

pub fn green_button_style(_theme: &Theme, status: Status) -> iced_widget::button::Style {
    button_style_inner(status, color!(0x77f277), color!(0x49d649), color!(0x348734))
}

pub fn gray_button_style(_theme: &Theme, status: Status) -> iced_widget::button::Style {
    button_style_inner(status, color!(0xa0a0a0), color!(0x787878), color!(0x505050))
}

fn button_style_inner(
    status: Status,
    color_light: Color,
    color_normal: Color,
    color_hard: Color,
) -> iced_widget::button::Style {
    iced_widget::button::Style {
        background: Some(Background::Color(match status {
            Status::Active => color_light,
            Status::Disabled => color_normal,
            Status::Hovered => color_normal,
            Status::Pressed => color_normal,
        })),
        text_color: color!(0xffffff),
        border: Border {
            color: match status {
                Status::Active => color_normal,
                Status::Disabled => color_hard,
                Status::Hovered => color_hard,
                Status::Pressed => color_hard,
            },
            width: match status {
                Status::Active => 1.0,
                Status::Disabled => 0.0,
                Status::Hovered => 2.0,
                Status::Pressed => 2.0,
            },
            radius: Radius::new(4),
        },
        shadow: Shadow::default(),
    }
}
