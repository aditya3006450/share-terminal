mod app;
use app::App;
pub mod components;
pub mod constants;
pub mod pages;
pub mod router;
pub mod services;
pub mod utils;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
