use dioxus::logger::tracing;

mod app;
mod components;
mod model;
mod net;

fn main() {
    dioxus::logger::init(tracing::Level::DEBUG).unwrap();
    dioxus::launch(app::App);
}
