use crate::basic_app::*;
use crate::constants::*;
use crate::shared::*;
use nwg::NativeUi;

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

    #[nwg_control(parent: tray_menu, text: "Auto update")]
    #[nwg_events(OnMenuItemSelected: [SystemTray::toggle_update])]
    tray_item_update: nwg::MenuItem,

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

        system_tray.first_time_init();
        system_tray.auto_update();

        nwg::dispatch_thread_events();

        system_tray
    }

    fn show_menu(&self) {
        self.refresh_startup();
        self.refresh_update();

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

    fn toggle_update(&self) {
        let Ok((key, _)) = self.get_reg_key() else {
            return;
        };

        let _: Result<(), std::io::Error> =
            key.set_value("AutoUpdate", &(!self.tray_item_update.checked() as u32));

        self.auto_update();
    }

    fn refresh_startup(&self) {
        let Ok(auto_launch) = self.get_auto_launch() else {
            return;
        };

        if let Ok(is_enabled) = auto_launch.is_enabled() {
            self.tray_item_startup.set_checked(is_enabled);
        }
    }

    fn refresh_update(&self) {
        let Ok((key, _)) = self.get_reg_key() else {
            return;
        };

        if let Ok(auto_update) = key.get_value::<u32, &str>("AutoUpdate") {
            self.tray_item_update.set_checked(auto_update > 0);
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

    fn get_reg_key(
        &self,
    ) -> Result<(winreg::RegKey, winreg::enums::RegDisposition), std::io::Error> {
        winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
            .create_subkey(std::path::Path::new("SOFTWARE").join(APP_NAME))
    }

    fn set_foreground_window(&self) -> bool {
        if let Some(hwnd) = unsafe { *WINDOW_HANDLE.lock().unwrap() } {
            unsafe { winapi::um::winuser::SetForegroundWindow(hwnd) };
            return true;
        }
        false
    }

    fn first_time_init(&self) {
        let Ok((key, _)) = self.get_reg_key() else {
            return;
        };

        if key.get_value::<u32, &str>("FirstTimeUse").is_err()
            && key.set_value("FirstTimeUse", &(true as u32)).is_err()
        {
            return;
        }

        let Ok(first_time_use) = key.get_value::<u32, &str>("FirstTimeUse") else {
            return;
        };

        if first_time_use == (false as u32) {
            return;
        }

        if key.set_value("FirstTimeUse", &(false as u32)).is_err() {
            return;
        }

        if let Ok(auto_launch) = self.get_auto_launch() {
            let _: Result<(), al::Error> = auto_launch.enable();
        }

        let _: Result<(), std::io::Error> = key.set_value("AutoUpdate", &(true as u32));
    }

    fn auto_update(&self) {
        let Ok((key, _)) = self.get_reg_key() else {
            return;
        };

        let Ok(auto_update) = key.get_value::<u32, &str>("AutoUpdate") else {
            return;
        };

        if auto_update > 0 {
            let _: Result<(), Box<su::errors::Error>> = self.update();
        }
    }

    fn update(&self) -> Result<(), Box<su::errors::Error>> {
        let status: su::Status = su::backends::github::Update::configure()
            .repo_owner("jqntn")
            .repo_name(APP_NAME)
            .bin_name(APP_NAME)
            .show_download_progress(true)
            .current_version(su::cargo_crate_version!())
            .build()?
            .update()?;

        println!("Update status: `{}`!", status.version());

        Ok(())
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
        nwg::stop_thread_dispatch();
    }
}
