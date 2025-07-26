pub mod root;
mod routine_common;
mod routine_detail;
mod routine_list;
mod routine_modify;
mod styles;

type AppUIElement =
    iced_core::Element<'static, crate::system::UIEvent, iced_core::Theme, iced_wgpu::Renderer>;

mod const_text {
    pub const TEXT_RUN_AT_STARTUP: &str = "run at startup";
    pub const TEXT_NOT_RUN_AT_STARTUP: &str = "not run at startup";
    pub const TEXT_RUN: &str = "run";
    pub const TEXT_STOP: &str = "stop";
    pub const TEXT_DELETE: &str = "delete";
    pub const TEXT_MODIFY: &str = "modify";
    pub const TEXT_DONE: &str = "done";
    pub const TEXT_BACK: &str = "back";
    pub const TEXT_CANCEL: &str = "cancel";
}
