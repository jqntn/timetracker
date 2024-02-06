use crate::basic_app::*;
use crate::constants::*;
use crate::shared::*;
use nwg::NativeUi;
use winapi::um::winuser::SetForegroundWindow;
use winreg::enums::*;

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

    #[nwg_control(parent: tray_menu, text: "Show records")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::show_window])]
    tray_item_open: nwg::MenuItem,

    #[nwg_control(parent: tray_menu)]
    separator0: nwg::MenuSeparator,

    #[nwg_control(parent: tray_menu, text: "Run at startup")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::toggle_startup])]
    tray_item_startup: nwg::MenuItem,

    #[nwg_control(parent: tray_menu)]
    separator1: nwg::MenuSeparator,

    #[nwg_control(parent: tray_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::exit])]
    tray_item_exit: nwg::MenuItem,
}

impl SystemTray {
    pub fn build() -> system_tray_ui::SystemTrayUi {
        let system_tray: system_tray_ui::SystemTrayUi =
            SystemTray::build_ui(SystemTray::default()).expect("Failed to build UI");

        system_tray.enable_startup_first_time();

        nwg::dispatch_thread_events();

        system_tray
    }

    fn show_menu(&self) {
        self.refresh_startup();

        let (x, y): (i32, i32) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn show_window(&self) {
        if self.set_foreground_window() {
            return;
        }

        let basic_app = BasicApp::build();

        unsafe { *WINDOW_HANDLE.lock().unwrap() = Some(basic_app.window().handle.hwnd().unwrap()) };

        self.set_foreground_window();

        nwg::dispatch_thread_events();
    }

    fn toggle_startup(&self) {
        let Ok(auto_launch) = self.get_auto_launch() else {
            return;
        };

        if !self.tray_item_startup.checked() {
            let _: Result<(), al::Error> = auto_launch.enable();
        } else {
            let _: Result<(), al::Error> = auto_launch.disable();
        }
    }

    fn refresh_startup(&self) {
        let Ok(auto_launch) = self.get_auto_launch() else {
            return;
        };

        if let Ok(is_enabled) = auto_launch.is_enabled() {
            self.tray_item_startup.set_checked(is_enabled);
        }
    }

    fn get_auto_launch(&self) -> Result<al::AutoLaunch, ()> {
        let Ok(exe_path) = std::env::current_exe() else {
            return Err(());
        };

        Ok(al::AutoLaunchBuilder::new()
            .set_app_name(APP_NAME)
            .set_app_path(exe_path.to_str().unwrap())
            .build()
            .unwrap())
    }

    fn set_foreground_window(&self) -> bool {
        if let Some(hwnd) = unsafe { *WINDOW_HANDLE.lock().unwrap() } {
            unsafe { SetForegroundWindow(hwnd) };
            return true;
        }
        false
    }

    fn enable_startup_first_time(&self) {
        if let Ok((_, disp)) = winreg::RegKey::predef(HKEY_CURRENT_USER)
            .create_subkey(std::path::Path::new("SOFTWARE").join(APP_NAME))
        {
            if disp == REG_CREATED_NEW_KEY {
                if let Ok(auto_launch) = self.get_auto_launch() {
                    let _: Result<(), al::Error> = auto_launch.enable();
                }
            }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
        nwg::stop_thread_dispatch();
    }
}
