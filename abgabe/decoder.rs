extern crate regex;

use regex::Regex;
use std::fmt;
use std::path::Path;

use visuals;
use decoder_class;
use decoder_usecase;
use decoding_to_visual;


pub struct Model {
    pub class_amount: i32,
    pub class_file_name: String,
    pub use_case_amount: i32,
    pub use_case_file_name: String,
}

impl Model {
    fn new (class_amount: i32, use_case_amount: i32, class_file_name: String, use_case_file_name: String) -> Model {
        Model {class_amount: class_amount, use_case_amount: use_case_amount, class_file_name: class_file_name, use_case_file_name: use_case_file_name }
    }
}


pub fn decode_input(given_input: String) -> Model {
    let diagram_regex = Regex::new(r"((Class|UseCase)~.*)").unwrap();
    let input = given_input.to_string();
    let mut class_diagram_list = Vec::new();
    let mut use_case_diagram_list = Vec::new();

    let diagram_strings = given_input.split("---");
    for dia_str in diagram_strings {
        if diagram_regex.is_match(dia_str.as_ref()) {
            let diagram_components: Vec<String> = dia_str.split(&"~".to_string()).map(|x| x.to_owned()).collect();
            if diagram_components[0] == "Class" {
                let class_return = decoder_class::decode_class_diagram(diagram_components[1].to_string());
                match class_return {
                    Some(class_return) => class_diagram_list.push(class_return),
                    None => continue,
                }
            } else if diagram_components[0] == "UseCase" {
                let uc_return = decoder_usecase::decode_use_case_diagram(diagram_components[1].to_string());
                match uc_return {
                    Some(uc_return) => use_case_diagram_list.push(uc_return),
                    None => continue,
                }
            }
        }
    }
    let model = Model::new(class_diagram_list.len() as i32, use_case_diagram_list.len() as i32,
                           "res/UML_class".to_string(), "res/UML_use_case".to_string());

    for j in 0..class_diagram_list.len() {
        decoding_to_visual::call_class_draw(&class_diagram_list[j].classes, &class_diagram_list[j].relations, j as i32)
    }


    for i in 0..use_case_diagram_list.len() {
        decoding_to_visual::call_use_case_draw(&use_case_diagram_list[i], i as i32);
    }

    return model;
}