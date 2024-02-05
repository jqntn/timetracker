use crate::basic_app::*;
use crate::constants::*;
use crate::shared::*;
use nwg::NativeUi;
use winapi::um::winuser::SetForegroundWindow;

#[derive(Default, nwd::NwgUi)]
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
    pub fn build() -> system_tray_ui::SystemTrayUi {
        let system_tray: system_tray_ui::SystemTrayUi =
            SystemTray::build_ui(SystemTray::default()).expect("Failed to build UI");

        nwg::dispatch_thread_events();

        system_tray
    }

    fn show_menu(&self) {
        let (x, y): (i32, i32) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn show_window(&self) {
        if self.try_set_foreground_window() {
            return;
        }

        let basic_app = BasicApp::build();

        unsafe { *WINDOW_HANDLE.lock().unwrap() = Some(basic_app.window().handle.hwnd().unwrap()) };

        self.try_set_foreground_window();

        nwg::dispatch_thread_events();
    }

    fn try_set_foreground_window(&self) -> bool {
        if let Some(hwnd) = unsafe { *WINDOW_HANDLE.lock().unwrap() } {
            unsafe { SetForegroundWindow(hwnd) };
            return true;
        }
        false
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
        nwg::stop_thread_dispatch();
    }
}
