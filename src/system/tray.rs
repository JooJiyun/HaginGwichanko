use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

use crate::{contexted_err, SResult};

const SYSTEM_TRAY_ICON_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/app-icon.ico");

pub enum SystemTrayEvent {
    Open,
    Quit,
    Start,
    Stop,
    Invalid,
}

pub struct SystemTrayHandle {
    pub tray_icon: Option<TrayIcon>,
    pub open_menu_item: MenuItem,
    pub quit_menu_item: MenuItem,
    pub start_menu_item: MenuItem,
    pub stop_menu_item: MenuItem,
}

impl Default for SystemTrayHandle {
    fn default() -> Self {
        Self {
            tray_icon: None,
            open_menu_item: MenuItem::new("open", true, None),
            quit_menu_item: MenuItem::new("exit", true, None),
            start_menu_item: MenuItem::new("start", true, None),
            stop_menu_item: MenuItem::new("stop", false, None),
        }
    }
}

impl SystemTrayHandle {
    pub fn init(&mut self) -> SResult<()> {
        let icon = load_system_tray_icon()?;

        let menu = Menu::new();
        menu.append_items(&[
            &self.open_menu_item,
            &self.quit_menu_item,
            &self.start_menu_item,
        ])
        .or_else(|e| contexted_err!("failed system tray menu", e))?;

        self.tray_icon = Some(
            TrayIconBuilder::new()
                .with_menu(Box::new(menu))
                .with_icon(icon)
                .with_title("netflix-skip-intro")
                .build()
                .or_else(|e| contexted_err!("failed create system tray builder", e))?,
        );

        Ok(())
    }

    /// menu event가 SystemTrayEvent 중 어떤 이벤트인지만 반환.
    ///
    /// menu item의 valid 상태를 바꾸지 않음!
    pub fn parse_event(&self, menu_event: MenuEvent) -> SystemTrayEvent {
        if menu_event.id() == self.open_menu_item.id() {
            return SystemTrayEvent::Open;
        }
        if menu_event.id() == self.quit_menu_item.id() {
            return SystemTrayEvent::Quit;
        }
        if menu_event.id() == self.start_menu_item.id() {
            return SystemTrayEvent::Start;
        }
        if menu_event.id() == self.stop_menu_item.id() {
            return SystemTrayEvent::Stop;
        }
        return SystemTrayEvent::Invalid;
    }

    pub fn set_menu_state(&mut self, running: bool) {
        self.start_menu_item.set_enabled(!running);
        self.stop_menu_item.set_enabled(running);
    }
}

fn load_system_tray_icon() -> SResult<Icon> {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(SYSTEM_TRAY_ICON_PATH)
            .or_else(|e| contexted_err!("failed load system tray icon image", e))?
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .or_else(|e| contexted_err!("failed create system tray icon", e))
}
