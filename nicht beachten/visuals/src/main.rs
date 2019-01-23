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
mod decoding_to_visual;
fn main() {

   // visuals::main();
   // gui::gui_main();


    gui::gui_main();



    let abc = decoder_usecase::decode_use_cases("2:schlafen:EP".to_string());
}