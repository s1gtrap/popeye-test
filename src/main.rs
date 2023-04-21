mod app;
pub(crate) mod data;
mod graph;
pub(crate) mod select;
pub(crate) mod set;

use app::App;

fn main() {
    wasm_log::init(wasm_log::Config::default());

    yew::Renderer::<App>::new().render();
}
