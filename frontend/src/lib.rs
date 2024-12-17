use wasm_bindgen::prelude::*;
use yew::Renderer;
use web_sys::console;

mod app;
mod home;
mod login;
mod signup;
mod profile;
mod recommendation;
mod components;
mod auth_context;
mod config;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Initializing Yew App...");
    console::log_1(&"run_app() called".into());
    Renderer::<app::App>::new().render();
}