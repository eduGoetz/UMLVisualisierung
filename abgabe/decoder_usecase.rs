extern crate regex;

use regex::Regex;
use std::fmt;


#[derive(Debug)]
pub struct UseCaseDiagramm {
    pub name: String,
    pub actors: Vec<Actor>,
    pub use_cases: Vec<UseCase>,
    pub use_case_realtions: Vec<UseCaseRelation>,
}

impl UseCaseDiagramm {
    fn new(name: String, actors: Vec<Actor>, use_cases: Vec<UseCase>, use_case_realations: Vec<UseCaseRelation>) -> UseCaseDiagramm {
        UseCaseDiagramm { name: name, actors: actors, use_cases: use_cases, use_case_realtions: use_case_realations }
    }
}


#[derive(Debug)]
pub struct Actor {
    pub id: i32,
    pub name: String,
    pub extends_from: Option<i32>,
    pub has_use_case: Vec<i32>,
}

impl Actor {
    fn new(id: i32, name: String, extends_from: Option<i32>, has_use_case: Vec<i32>) -> Actor {
        Actor { id: id, name: name, extends_from: extends_from, has_use_case: has_use_case }
    }
}


#[derive(Debug)]
pub struct UseCase {
    pub id: i32,
    pub name: String,
    pub is_extension_point: bool,
}

impl UseCase {
    fn new(id: i32, name: String, is_extension_point: bool) -> UseCase {
        UseCase { id: id, name: name, is_extension_point: is_extension_point }
    }
}


#[derive(Debug)]
pub struct UseCaseRelation {
    pub relation_type: UseCaseRelationType,
    pub from: i32,
    pub to: i32,
}

impl UseCaseRelation {
    fn new(relation_type: UseCaseRelationType, from: i32, to: i32) -> UseCaseRelation {
        UseCaseRelation { relation_type: relation_type, from: from, to: to }
    }
}


#[derive(Debug)]
pub enum UseCaseRelationType {
    Extends,
    Include,
}


fn decode_actors(actors_str: String) -> Vec<Actor> {
    let actor_regex = Regex::new(r"(\d+:\w+:(\d+)?:(\d+( )?)?)").unwrap();
    let mut actors_return = Vec::new();

    let actors_strings = actors_str.split(",");
    for ac_str in actors_strings {
        if actor_regex.is_match(ac_str.as_ref()) {
            let mut ac_uc_str = String::new();
            let actor_components: Vec<String> = ac_str.split(&":".to_string()).map(|x| x.to_owned()).collect();
            let mut actor_use_cases = Vec::new();

            let mut extends_from = None;
            if (actor_components[2].to_string() != "") {
                extends_from = Some(actor_components[2].to_string().parse::<i32>().unwrap());
            }

            ac_uc_str = actor_components[3].to_string();
            let actor_use_cases_str = ac_uc_str.split(" ");
            for ac_to_use_str in actor_use_cases_str {
                actor_use_cases.push(ac_to_use_str.to_string().parse::<i32>().unwrap());
            }

            actors_return.push(Actor::new(actor_components[0].to_string().parse::<i32>().unwrap(), actor_components[1].to_string(),
                                          extends_from, actor_use_cases));
        }
    }
    return actors_return;
}


pub fn decode_use_cases(use_cases_str: String) -> Vec<UseCase> {
    let use_case_regex = Regex::new(r"((\d+:(\w|\s)+:(EP)?)(,)?)").unwrap();
    let mut use_case_return = Vec::new();

    let use_case_strings = use_cases_str.split(",");
    for uc_str in use_case_strings {
        println!("{}", uc_str);
        if use_case_regex.is_match(uc_str.as_ref()) {
            let use_case_components: Vec<String> = uc_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            let num = use_case_components[0].to_string().parse::<i32>().unwrap();
            use_case_return.push(UseCase::new(num,
                                              use_case_components[1].to_string(), use_case_components[2].to_string() != ""));
        }
    }

    return use_case_return;
}


fn decode_use_case_relations(use_cases_relations_str: String) -> Vec<UseCaseRelation> {
    let relation_use_case_regex = Regex::new(r"((Extends|Include):\d+->\d+)").unwrap();
    let mut relation_uc_return = Vec::new();

    let use_cases_realtions_strings = use_cases_relations_str.split(",");
    for rel_uc_str in use_cases_realtions_strings {
        if relation_use_case_regex.is_match(rel_uc_str.as_ref()) {
            let relation_uc_components: Vec<String> = rel_uc_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            let mut relation_type_uc: UseCaseRelationType;
            match relation_uc_components[0].as_ref() {
                "Extends" => relation_type_uc = UseCaseRelationType::Extends,
                "Include" => relation_type_uc = UseCaseRelationType::Include,
                _ => continue,
            }

            let mut from_to_vec: Vec<i32> = Vec::new();
            for f_t_uc in relation_uc_components[1].split("->") {
                from_to_vec.push(f_t_uc.parse::<i32>().unwrap());
            }

            relation_uc_return.push(UseCaseRelation::new(relation_type_uc, from_to_vec[0], from_to_vec[1]));
        }
    }
    return relation_uc_return;
}


pub fn decode_use_case_diagram(diagram_input: String) -> Option<UseCaseDiagramm> {
    let diagram_input_regex = Regex::new(r"(\w+;.*;.*;.*)").unwrap();
    let diagram_input = diagram_input.to_string();

    if diagram_input_regex.is_match(diagram_input.as_ref()) {
        let use_case_diagram_comp: Vec<String> = diagram_input.split(&";".to_string()).map(|x| x.to_owned()).collect();
        return Some(UseCaseDiagramm::new(use_case_diagram_comp[0].to_string(), decode_actors(use_case_diagram_comp[1].to_string()),
                                         decode_use_cases(use_case_diagram_comp[2].to_string()), decode_use_case_relations(use_case_diagram_comp[3].to_string())));
    }
    return None;
}