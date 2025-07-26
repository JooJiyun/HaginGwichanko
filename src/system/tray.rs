use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder, TrayIconEvent};

use crate::system::ICON_PATH;
use crate::{contexted_err, SResult};

pub enum SystemTrayEvent {
    Open,
    Quit,
    Invalid,
}

pub struct SystemTrayHandle {
    pub tray_icon: Option<TrayIcon>,
    pub open_menu_item: MenuItem,
    pub quit_menu_item: MenuItem,
}

impl Default for SystemTrayHandle {
    fn default() -> Self {
        Self {
            tray_icon: None,
            open_menu_item: MenuItem::new("open", true, None),
            quit_menu_item: MenuItem::new("exit", true, None),
        }
    }
}

impl SystemTrayHandle {
    pub fn init(&mut self) -> SResult<()> {
        let icon = load_system_tray_icon()?;

        let menu = Menu::new();
        menu.append_items(&[&self.open_menu_item, &self.quit_menu_item])
            .or_else(|e| contexted_err!("failed system tray menu", e))?;

        self.tray_icon = Some(
            TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_icon(icon)
                .with_title("hagin-gwichanko")
                .with_menu_on_left_click(false)
                .build()
                .or_else(|e| contexted_err!("failed create system tray builder", e))?,
        );

        Ok(())
    }

    pub fn parse_menu_event(&self, menu_event: MenuEvent) -> SystemTrayEvent {
        if menu_event.id() == self.open_menu_item.id() {
            return SystemTrayEvent::Open;
        }
        if menu_event.id() == self.quit_menu_item.id() {
            return SystemTrayEvent::Quit;
        }
        return SystemTrayEvent::Invalid;
    }

    pub fn parse_icon_event(&self, icon_event: TrayIconEvent) -> SystemTrayEvent {
        match icon_event {
            TrayIconEvent::DoubleClick {
                id: _,
                position: _,
                rect: _,
                button: _,
            } => SystemTrayEvent::Open,
            _ => SystemTrayEvent::Invalid,
        }
    }
}

fn load_system_tray_icon() -> SResult<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(ICON_PATH)
            .or_else(|e| contexted_err!("failed load system tray icon image", e))?
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .or_else(|e| contexted_err!("failed create system tray icon", e))
}
