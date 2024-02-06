use std::sync::Mutex;
use winapi::shared::windef::HWND;

pub static mut WINDOW_HANDLE: Mutex<Option<HWND>> = Mutex::new(None);
