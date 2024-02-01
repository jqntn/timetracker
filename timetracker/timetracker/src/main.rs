#![allow(unused_must_use)]
#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use winapi::um::winuser::SetProcessDPIAware;

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource(source_file: Some("res/icon/hourglass-96.ico"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some("timetracker"))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_menu], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "Open")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::show_window])]
    tray_item_open: nwg::MenuItem,

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
        let _: nwg::MessageChoice = nwg::simple_message("Hello", "Hello, World!");
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    let _: i32 = unsafe { SetProcessDPIAware() };

    nwg::init().expect("Failed to init Native Windows GUI");

    let _ui: system_tray_ui::SystemTrayUi =
        SystemTray::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
