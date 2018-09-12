extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::console::ConsoleService;

mod view;

fn main() {
    stdweb::initialize();
    yew::initialize();
    ConsoleService::new().log("Hello from Rust!");
    App::<view::View>::new().mount_to_body();
    yew::run_loop();
}
