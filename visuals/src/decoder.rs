extern crate regex;



use regex::Regex;

use std::fmt;

use std::path::Path;



use visuals;

use decoder_class;

use decoder_usecase;



pub fn decode_input(given_input: String) -> i32 {

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

    for j in 0..class_diagram_list.len() {

        call_class_draw(&class_diagram_list[j].classes, &class_diagram_list[j].relations, j as i32)

    }





    for i in &use_case_diagram_list {

        println!("{:?}", i);

    }



    return class_diagram_list.len() as i32;

}



fn call_class_draw(class_list: &Vec<decoder_class::Class>, relation_list: &Vec<decoder_class::Relation>, class_number: i32) {

    let path_first_part = "res/UML_visual_result";

    let path_ending = ".png";

    let path_str = [path_first_part, path_ending].join(&class_number.to_string());

    let path = Path::new(&path_str);



    let mut image = visuals::erstelle_image();

    for i in class_list {

        let mut klassentyp = "";

        if let decoder_class::ClassType::Class = i.class_type {

            klassentyp = "Class";

        } else if let decoder_class::ClassType::Abstract = i.class_type {

            klassentyp = "Abstract";

        } else {

            klassentyp = "Interface";

        }



        image = visuals::klasse(i.name.as_ref(), klassentyp, image.clone(), path, i.class_id, &i.attributes, &i.methods);

    }



    for j in relation_list {

        let mut pfeiltyp = "";

        if let decoder_class::RelationType::Vererbung = j.relation_type {

            pfeiltyp = "ver";

        } else if let decoder_class::RelationType::Aggregation = j.relation_type {

            pfeiltyp = "agg";

        } else if let decoder_class::RelationType::Komposition = j.relation_type {

            pfeiltyp = "kompo";

        } else if let decoder_class::RelationType::Assoziation = j.relation_type {

            pfeiltyp = "asso";

        } else if let decoder_class::RelationType::GerAssoziation = j.relation_type {

            pfeiltyp = "ge_asso";

        } else if let decoder_class::RelationType::Implementierung = j.relation_type {

            pfeiltyp = "imple";

        } else {

            pfeiltyp = "abh";

        }



        image = visuals::zeichne_pfeil(image.clone(), path, pfeiltyp, j.from, j.to, &j.from_multiplicity, &j.to_multiplicity);

    }

}