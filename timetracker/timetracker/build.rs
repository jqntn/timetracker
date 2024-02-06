extern crate winapi;
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon_with_id("res/icon/timetracker-96.ico", "APPICON");
    res.set_manifest_file("exe.manifest");
    res.set_language(winapi::um::winnt::LANG_ENGLISH);
    res.compile().unwrap();
}
