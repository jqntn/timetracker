#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use single_instance::SingleInstance;
use winapi::{shared::windef::HWND, um::winuser::SetForegroundWindow};

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("APPICON"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some("timetracker"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_window], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Show timetracker")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::show_window])]
    tray_item_open: nwg::MenuItem,

    #[nwg_control(parent: tray_menu)]
    separator: nwg::MenuSeparator,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item_exit: nwg::MenuItem,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y): (i32, i32) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn show_window(&self) {
        if !self.try_set_foreground_window() {
            return;
        }

        let _basic_app: basic_app_ui::BasicAppUi =
            BasicApp::build_ui(Default::default()).expect("Failed to build UI");

        unsafe { WINDOW_HANDLE = Some(_basic_app.window.handle.hwnd().unwrap()) };
        let _: bool = self.try_set_foreground_window();

        nwg::dispatch_thread_events();
    }

    fn try_set_foreground_window(&self) -> bool {
        if let Some(hwnd) = unsafe { WINDOW_HANDLE } {
            let _: i32 = unsafe { SetForegroundWindow(hwnd) };
            return false;
        }
        true
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (WINDOW_SIZE.0 , WINDOW_SIZE.1), position: ((nwg::Monitor::width() - WINDOW_SIZE.0) / 2, (nwg::Monitor::height() - WINDOW_SIZE.1) / 2), title: "timetracker", icon: Some(&data.icon))]
    #[nwg_events( OnWindowClose: [BasicApp::exit] )]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("APPICON"))]
    icon: nwg::Icon,
}

impl BasicApp {
    fn exit(&self) {
        unsafe { WINDOW_HANDLE = None };

        nwg::stop_thread_dispatch();
    }
}

const WINDOW_SIZE: (i32, i32) = (800, 450);
static mut WINDOW_HANDLE: Option<HWND> = None;

fn main() {
    let instance: SingleInstance = SingleInstance::new("timetracker").unwrap();
    assert!(instance.is_single());

    nwg::init().expect("Failed to init Native Windows GUI");

    let _system_tray: system_tray_ui::SystemTrayUi =
        SystemTray::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
