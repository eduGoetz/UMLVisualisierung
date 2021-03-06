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
mod visuals_class;
mod visuals_case;
mod decoding_to_visual;

fn main() {

    let method_regex = Regex::new(r"^((public|private|protected|package)?:(static)?:(final)?:(\w+):(\w+):(.*)?(,?))*$").unwrap();
    println!("{}",method_regex.is_match(":::voiddm:achWas:int=nummer"));

    //visuals::use_case_hard();
    gui::gui_main();

    let abc = decoder_usecase::decode_use_cases("2:schlafen:EP".to_string());
}