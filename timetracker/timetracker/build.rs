extern crate embed_resource;

fn main() {
    embed_resource::compile("timetracker-manifest.rc", embed_resource::NONE);
}
