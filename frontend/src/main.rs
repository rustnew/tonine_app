use yew::Renderer;
use yew::prelude::*;


pub mod form;
pub mod home_page;
pub mod login;
pub mod tontine;

use  form::Form;
use  home_page::HomePage;
use tontine::tontine::TontinePage;

fn  main() {
    console_log::init().expect("Logger initialization failed");
    Renderer::<TontinePage>::new().render();
}