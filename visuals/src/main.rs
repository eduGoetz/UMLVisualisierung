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
mod gui;
mod visuals;

fn main() {

    visuals::erstelle_use_case();
    gui::gui_main();
}