use crate::constants::*;
use crate::shared::*;
use nwg::NativeUi;

#[derive(Default, nwd::NwgUi)]
pub struct BasicApp {
    #[nwg_control(
        size: (WINDOW_SIZE.0 , WINDOW_SIZE.1),
        position: ((nwg::Monitor::width() - WINDOW_SIZE.0) / 2, (nwg::Monitor::height() - WINDOW_SIZE.1) / 2),
        title: APP_NAME,
        icon: Some(&data.icon))]
    #[nwg_events(OnWindowClose: [BasicApp::exit])]
    window: nwg::Window,

    #[nwg_resource]
    embed: nwg::EmbedResource,

    #[nwg_resource(source_embed: Some(&data.embed), source_embed_str: Some("APPICON"))]
    icon: nwg::Icon,
}

impl BasicApp {
    pub fn build() -> basic_app_ui::BasicAppUi {
        BasicApp::build_ui(BasicApp::default()).expect("Failed to build UI")
    }

    pub fn window(&self) -> &nwg::Window {
        &self.window
    }

    fn exit(&self) {
        unsafe { *WINDOW_HANDLE.lock().unwrap() = None };

        nwg::stop_thread_dispatch();
    }
}
