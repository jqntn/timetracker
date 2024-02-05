#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
extern crate single_instance as si;
extern crate winapi;

mod basic_app;
mod constants;
mod shared;
mod system_tray;

use crate::constants::*;
use crate::system_tray::*;

fn main() {
    let instance: si::SingleInstance = si::SingleInstance::new(APP_NAME).unwrap();
    assert!(instance.is_single());

    nwg::init().expect("Failed to init Native Windows GUI");

    SystemTray::build();
}
