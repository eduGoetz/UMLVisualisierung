extern crate regex;
extern crate gtk;
use regex::Regex;
use gtk::*;

mod decoder;
mod gui;

fn main() {

    gui::gui_main();
}