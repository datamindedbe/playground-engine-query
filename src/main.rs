#![allow(dead_code)]

mod data;
mod ui;

use std::panic;

fn main() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let data = data::parse_data_from_static_yamls();

    ui::mount_ui_to_body(&data);
}
