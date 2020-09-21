#![recursion_limit = "512"]

mod app;
mod utils;

use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());

    let mount_div = utils::get_root_element();

    yew::initialize();
    yew::App::<app::App>::new().mount(mount_div);
    yew::run_loop();

    Ok(())
}
