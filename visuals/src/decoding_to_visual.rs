use std::fmt;
use std::path::Path;

use decoder;
use decoder_class;
use decoder_usecase;
use visuals;


pub fn call_class_draw(class_list: &Vec<decoder_class::Class>, relation_list: &Vec<decoder_class::Relation>, class_number: i32) {
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


pub fn call_use_case_draw(use_case_diagram: &decoder_usecase::UseCaseDiagramm) {
    let mut image = visuals::erstelle_image();

    image = visuals::create_system_and_akteur(image.clone(), &use_case_diagram.name, &use_case_diagram.actors);
    image = visuals::create_cases(image.clone(), &use_case_diagram.use_cases);


}