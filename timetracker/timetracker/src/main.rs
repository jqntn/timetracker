#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use single_instance::SingleInstance;
use std::sync::Mutex;
use winapi::{shared::windef::HWND, um::winuser::SetForegroundWindow};

const APP_NAME: &str = "timetracker";
const WINDOW_SIZE: (i32, i32) = (800, 450);

static mut WINDOW_HANDLE: Mutex<Option<HWND>> = Mutex::new(None);

#[derive(Default, NwgUi)]
pub struct SystemTray {
    #[nwg_control]
    window: nwg::MessageWindow,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("APPICON"))]
    icon: nwg::Icon,

    #[nwg_control(icon: Some(&data.icon), tip: Some(APP_NAME))]
    #[nwg_events(MousePressLeftUp: [SystemTray::show_window], OnContextMenu: [SystemTray::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: &format!("Show {}", APP_NAME))]
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

        unsafe { *WINDOW_HANDLE.lock().unwrap() = Some(_basic_app.window.handle.hwnd().unwrap()) };
        let _: bool = self.try_set_foreground_window();

        nwg::dispatch_thread_events();
    }

    fn try_set_foreground_window(&self) -> bool {
        if let Some(hwnd) = unsafe { *WINDOW_HANDLE.lock().unwrap() } {
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
    #[nwg_control(
        size: (WINDOW_SIZE.0 , WINDOW_SIZE.1),
        position: ((nwg::Monitor::width() - WINDOW_SIZE.0) / 2, (nwg::Monitor::height() - WINDOW_SIZE.1) / 2),
        title: APP_NAME,
        icon: Some(&data.icon))]
    #[nwg_events( OnWindowClose: [BasicApp::exit] )]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("APPICON"))]
    icon: nwg::Icon,
}

impl BasicApp {
    fn exit(&self) {
        unsafe { *WINDOW_HANDLE.lock().unwrap() = None };

        nwg::stop_thread_dispatch();
    }
}

struct App {}

impl App {
    fn new() -> Self {
        nwg::init().expect("Failed to init Native Windows GUI");

        let _system_tray: system_tray_ui::SystemTrayUi =
            SystemTray::build_ui(Default::default()).expect("Failed to build UI");

        nwg::dispatch_thread_events();

        Self {}
    }
}

fn main() {
    let instance: SingleInstance = SingleInstance::new(APP_NAME).unwrap();
    assert!(instance.is_single());

    let _app: App = App::new();
}
