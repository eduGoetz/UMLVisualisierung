extern crate regex;
extern crate gtk;
extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate conv;
use regex::Regex;
use gtk::*;



mod decoder;
mod gui;
mod visuals;

fn main() {

    gui::gui_main();
}