extern crate regex;
extern crate gtk;
extern crate image;
extern crate imageproc;
extern crate rusttype;
extern crate conv;
extern crate webbrowser;
use regex::Regex;
use gtk::*;



mod decoder;
mod decoder_class;
mod decoder_usecase;
mod gui;
mod visuals;

fn main() {

    //visuals::use_case_hard();
    gui::gui_main();
}