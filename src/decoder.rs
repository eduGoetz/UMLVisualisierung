extern crate regex;
use regex::Regex;
use std::fmt;
use std::path::Path;

use visuals;

#[derive(Debug)]
pub struct ClassDiagram {
    pub classes: Vec<Class>,
    pub relations: Vec<Relation>,
}

impl ClassDiagram {
    fn new(classes: Vec<Class>, relations: Vec<Relation>) -> ClassDiagram {
        ClassDiagram {classes: classes, relations: relations}
    }
}

#[derive(Debug)]
pub struct Class {
    pub class_id: i32,
    pub class_type: ClassType,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub methods: Vec<Method>,
}

impl Class {
    fn new(class_id: i32, class_type: ClassType, name: String, attributes: Vec<Attribute>, methods: Vec<Method>) -> Class {
        Class {class_id: class_id, class_type: class_type, name: name, attributes: attributes, methods: methods}
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut params_str = "".to_string();
        for i in &self.attributes{
            params_str = [params_str, i.to_string()].join("\n");
        }

        let mut methods_str = "".to_string();
        for j in &self.methods{
            methods_str = [methods_str, j.to_string()].join("\n");
        }
        write!(f, "{}, {} \n {}", self.class_type.to_string(), params_str, methods_str);
        Ok(())
    }
}


#[derive(Debug)]
pub struct Attribute {
    pub access_modifier: AccessModifier,
    pub is_static: bool,
    pub is_final: bool,
    pub data_type: String,
    pub name: String,
}

impl Attribute {
    fn new(access_modifier: AccessModifier, is_static: bool, is_final: bool, data_type: String, name: String) -> Attribute {
        Attribute { access_modifier: access_modifier, is_static: is_static, is_final: is_final, data_type: data_type, name: name}
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_mod_str = "".to_string();
        if let AccessModifier::Public = self.access_modifier {
            access_mod_str = "+".to_string();
        } else if let AccessModifier::Private = self.access_modifier {
            access_mod_str = "-".to_string();
        } else if let AccessModifier::Protected = self.access_modifier {
            access_mod_str = "#".to_string();
        } else if let AccessModifier::Package = self.access_modifier {
            access_mod_str = "~".to_string();
        } else {
            access_mod_str = "".to_string();
        }

        if(self.is_final == true){
            write!(f, "{}{} : {}{}", access_mod_str, self.name.to_uppercase(), self.data_type, "{readOnly}");
        } else {
            write!(f, "{}{} : {}", access_mod_str, self.name, self.data_type);
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct Parameter {
    pub data_type: String,
    pub name: String,
}

impl Parameter {
    fn new(data_type: String, name: String) -> Parameter {
        Parameter { data_type: data_type, name: name}
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.data_type);
        Ok(())
    }
}


#[derive(Debug)]
pub struct Method {
    pub access_modifier: AccessModifier,
    pub is_static: bool,
    pub is_final: bool,
    pub return_type: String,
    pub name: String,
    pub parameters: Vec<Parameter>,
}

impl Method{
    fn new(access_modifier: AccessModifier, is_static: bool, is_final: bool, return_type: String, name: String, parameters: Vec<Parameter>) -> Method{
        Method { access_modifier: access_modifier, is_static: is_static, is_final: is_final, return_type: return_type, name: name, parameters: parameters}
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut access_mod_str = "".to_string();
        if let AccessModifier::Public = self.access_modifier {
            access_mod_str = "+".to_string();
        } else if let AccessModifier::Private = self.access_modifier {
            access_mod_str = "-".to_string();
        } else if let AccessModifier::Protected = self.access_modifier {
            access_mod_str = "#".to_string();
        } else if let AccessModifier::Package = self.access_modifier {
            access_mod_str = "~".to_string();
        } else {
            access_mod_str = "".to_string();
        }

        let mut params_str = "".to_string();
        if(!(&self.parameters).is_empty()) {
            for i in &self.parameters {
                params_str = [params_str, i.to_string()].join(", ");
            }
            params_str = params_str.trim_left_matches(", ").to_string();
        }

        if(self.is_final == true){
            write!(f, "{}{}({}):{}{}", access_mod_str, self.name.to_uppercase(), params_str, self.return_type, "{readOnly}");
        } else {
            write!(f, "{}{}({}):{}", access_mod_str, self.name, params_str, self.return_type,);
        }
        Ok(())
    }
}


#[derive(Debug)]
pub struct Relation {
    pub relation_type: RelationType,
    pub from: i32,
    pub to: i32,
    pub from_multiplicity: String,
    pub to_multiplicity: String,
}

impl Relation{
    fn new(relation_type: RelationType, from: i32, to: i32, from_multiplicity: String, to_multiplicity: String) -> Relation{
        Relation { relation_type: relation_type, from: from, to: to, from_multiplicity: from_multiplicity, to_multiplicity: to_multiplicity}
    }
}


#[derive(Debug)]
pub struct UseCaseDiagramm {
    pub name: String,
    pub actors: Vec<Actor>,
    pub use_cases: Vec<UseCase>,
    pub use_case_realtions: Vec<UseCaseRelation>,
}

impl UseCaseDiagramm {
    fn new(name: String, actors: Vec<Actor>, use_cases: Vec<UseCase>, use_case_realations: Vec<UseCaseRelation>) -> UseCaseDiagramm{
        UseCaseDiagramm { name: name, actors: actors, use_cases: use_cases, use_case_realtions: use_case_realations}
    }
}


#[derive(Debug)]
pub struct Actor {
    pub id: i32,
    pub name: String,
    pub extends_from: Option<i32>,
    pub has_use_case: Vec<i32>,
}

impl Actor{
    fn new(id: i32, name: String, extends_from: Option<i32>, has_use_case: Vec<i32>) -> Actor{
        Actor {id: id, name: name, extends_from: extends_from, has_use_case: has_use_case}
    }
}


#[derive(Debug)]
pub struct UseCase {
    pub id: i32,
    pub name: String,
    pub is_extension_point: bool,
}

impl UseCase{
    fn new(id: i32, name: String, is_extension_point: bool) -> UseCase {
        UseCase {id: id, name: name, is_extension_point: is_extension_point}
    }
}


#[derive(Debug)]
pub struct UseCaseRelation {
    pub relation_type: UseCaseRelationType,
    pub from: i32,
    pub to: i32,
}

impl UseCaseRelation{
    fn new(relation_type: UseCaseRelationType, from: i32, to: i32) -> UseCaseRelation {
        UseCaseRelation {relation_type: relation_type, from: from, to: to}
    }
}


#[derive(Debug)]
pub enum UseCaseRelationType{
    Extends,
    Include,
}

#[derive(Debug)]
pub enum ClassType{
    Class,
    Interface,
    Abstract,
}

impl fmt::Display for ClassType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClassType::Class => write!(f, "Class"),
            ClassType::Interface => write!(f, "Interface"),
            ClassType::Abstract => write!(f, "Abstract"),
        }
    }
}
#[derive(Debug)]
pub enum AccessModifier{
    Public,
    Private,
    Protected,
    Package,
    Default,
}
#[derive(Debug)]
pub enum RelationType{
    Vererbung,
    Aggregation,
    Komposition,
    Assoziation,
    GerAssoziation,
    Implementierung,
    Ableitung,
}


fn decode_classes(classes_str: String) -> (Vec<Class>, String){
    let classes_regex = Regex::new(r"(\d+;((Class)|(Interface)|(Abstract));(\w+);(.*);(.*)(/?))+").unwrap();

    let mut errors = "".to_string();
    let mut class_list_return = Vec::new();
    let classes_strings = classes_str.split("/");
    for cla_str in classes_strings {
        if classes_regex.is_match(cla_str.as_ref()) {
            let class_components_vec: Vec<String> = cla_str.split(&";".to_string()).map(|x| x.to_owned()).collect();

            let mut class_type: ClassType;
            match class_components_vec[1].as_ref() {
                "Class" => class_type = ClassType::Class,
                "Interface" => class_type = ClassType::Interface,
                "Abstract" => class_type = ClassType::Abstract,
                _ => {
                    errors = [errors, "Der Klassentyp ist falsch, mind. 1 Klasse übersprungen.\n".to_string()].join("");
                    continue
                },
            }

            errors = [errors, decode_attributes(class_components_vec[3].to_string()).1].join("");
            errors = [errors, decode_methods(class_components_vec[4].to_string()).1].join("");

            class_list_return.push(Class::new(class_components_vec[0].parse::<i32>().unwrap(), class_type,
                                              class_components_vec[2].to_string(), decode_attributes(class_components_vec[3].to_string()).0,
                                              decode_methods(class_components_vec[4].to_string()).0));
        } else {
            if (cla_str != "") {
                errors = [errors, "Das Klassenlayout ist falsch, mind. 1 Klasse übersprungen.\n".to_string()].join("");
            }
        }
    }
    return (class_list_return, errors);
}


fn decode_attributes(attributes_str: String) -> (Vec<Attribute>, String){
    let attribute_regex = Regex::new(r"((public|private|protected|package)?:(static)?:(final)?:(\w+):(\w+)(,?))*").unwrap();
    let mut errors = "".to_string();
    let mut class_attributes_return = Vec::new();

    let attributes_strings = attributes_str.split(",");
    for attr_str in attributes_strings {
        if attribute_regex.is_match(attr_str.as_ref()){

            let attribute_components: Vec<String> = attr_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            let mut attribute_access_modifier: AccessModifier;
            match attribute_components[0].as_ref(){
                "public" => attribute_access_modifier = AccessModifier::Public,
                "private" => attribute_access_modifier =  AccessModifier::Private,
                "protected" => attribute_access_modifier = AccessModifier::Protected,
                "package" => attribute_access_modifier = AccessModifier::Package,
                "" => attribute_access_modifier = AccessModifier::Default,
                _ => {
                    errors = [errors, "Der Zugriffsmodifikator in einem Attribut ist falsch, mind, ein Attribut übersprungen.\n".to_string()].join("");
                    continue
                },
            }

            if (!(attribute_components.is_empty()) && attribute_components.len()==5) {
                class_attributes_return.push(Attribute::new(attribute_access_modifier, attribute_components[1].to_string() != "",
                                                            attribute_components[2].to_string() != "", attribute_components[3].to_string(),
                                                            attribute_components[4].to_string()))
            } else {
                if attribute_components.is_empty() {
                    errors = [errors, "Das Attributlayout ist falsch, mind. ein Attribut übersprungen.\n".to_string()].join("");
                } else {
                    errors = [errors, "WARNUNG: Attributfelder wurden leer gelassen.\n".to_string()].join("");
                }
            }
        } else {
            errors = [errors, "Das Attributlayout ist falsch, mind. ein Attribut übersprungen.\n".to_string()].join("");
        }
    }
    return (class_attributes_return, errors);
}

//private:static::void:getNumber:int=number String=wort
fn decode_methods(methods_str: String) -> (Vec<Method>, String){
    let method_regex = Regex::new(r"((public|private|protected|package)?:(static)?:(final)?:(\w+):(\w+):(.*)?(,?))*").unwrap();
    let mut errors = "".to_string();

    let mut class_methods_return = Vec::new();

    let method_strings = methods_str.split(",");
    for met_str in method_strings {
        if met_str == "" {
            errors = [errors, "WARNUNG: Methodenfelder wurden leer gelassen.\n".to_string()].join("");
            continue;
        }
        if method_regex.is_match(met_str.as_ref()){
            let method_components: Vec<String> = met_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            let mut method_access_modifier: AccessModifier;
            match method_components[0].as_ref(){
                "public" => method_access_modifier = AccessModifier::Public,
                "private" => method_access_modifier =  AccessModifier::Private,
                "protected" => method_access_modifier = AccessModifier::Protected,
                "package" => method_access_modifier = AccessModifier::Package,
                "" => method_access_modifier = AccessModifier::Default,
                _ => {
                    errors = [errors, "Der Zugriffsmodifikator in einer Methode ist falsch, mind, eine Methode übersprungen.\n".to_string()].join("");
                    continue
                },
            }

            errors = [errors, decode_parameters(method_components[5].to_string()).1.to_string()].join("");

            if (!(method_components.is_empty()) && method_components.len()>=5) {
                class_methods_return.push(Method::new(method_access_modifier, method_components[1].to_string() != "",
                                                      method_components[2].to_string() != "", method_components[3].to_string(),
                                                      method_components[4].to_string(), decode_parameters(method_components[5].to_string()).0));
            } else{
                if method_components.is_empty() {
                    errors = [errors, "Das Methodenlayout ist falsch, mind. eine Methode übersprungen.\n".to_string()].join("");
                } else {
                    errors = [errors, "WARNUNG: Methodenfelder wurden leer gelassen.\n".to_string()].join("");
                }
            }
        } else {
            errors = [errors, "Das Methodenlayout ist falsch, mind. eine Methode übersprungen.\n".to_string()].join("");
        }
    }
    return (class_methods_return, errors);
}


fn decode_parameters(param_str: String) -> (Vec<Parameter>, String){
    let param_regex = Regex::new(r"((w+)=(w+)( )?)*").unwrap();
    let mut errors = "".to_string();
    let mut class_methods_param_return = Vec::new();

    let param_strings = param_str.split(" ");
    for par_str in param_strings {
        if param_regex.is_match(par_str.as_ref()){
            let single_params: Vec<String> = par_str.split(&"=".to_string()).map(|x| x.to_owned()).collect();

            if (!(single_params.is_empty()) && single_params.len()==2) {
                class_methods_param_return.push(Parameter::new(single_params[0].to_string(), single_params[1].to_string()));
            } else {
                if single_params.is_empty() {
                    errors = [errors, "Das Parameterlayout ist falsch, mind. ein Parameter übersprungen.\n".to_string()].join("");
                } else {
                    errors = [errors, "WARNUNG: Parameterfelder wurden leer gelassen.\n".to_string()].join("");
                }
            }
        } else {
            errors = [errors, "Das Parameterlayout ist falsch, mind. ein Parameter übersprungen.\n".to_string()].join("");
        }
    }
    return (class_methods_param_return, errors);
}


fn decode_relations(relations_str: String) -> Vec<Relation>{
    let relation_regex = Regex::new(r"((V|A|gA|AG|K|I|AB);\d+->\d+;((\d+|\w+|\*):(\d+|\w+|\*))?(,)?)*").unwrap();
    let mut relations_return = Vec::new();

    let relation_strings = relations_str.split(",");
    for rel_str in relation_strings {
        if relation_regex.is_match(rel_str.as_ref()){
            let relation_components: Vec<String> = rel_str.split(&";".to_string()).map(|x| x.to_owned()).collect();

            let mut relation_type: RelationType;
            match relation_components[0].as_ref(){
                "V" => relation_type = RelationType::Vererbung,
                "A" => relation_type = RelationType::Assoziation,
                "gA" => relation_type = RelationType::GerAssoziation,
                "AG" => relation_type = RelationType::Aggregation,
                "K" => relation_type = RelationType::Komposition,
                "I" => relation_type = RelationType::Implementierung,
                "AB" => relation_type = RelationType::Ableitung,
                _ => continue,
            }

            let mut from_to_vec: Vec<i32> = Vec::new();
            for f_t in relation_components[1].split("->"){
                from_to_vec.push(f_t.parse::<i32>().unwrap());
            }

            let mut multi_from_to_vec: Vec<String> = Vec::new();
            if relation_components[2] != "" {
                for m_f_t in relation_components[2].split(":"){
                    multi_from_to_vec.push(m_f_t.to_string());
                }
            } else {
                multi_from_to_vec.push("".to_string());
                multi_from_to_vec.push("".to_string());
            }

            relations_return.push(Relation::new(relation_type, from_to_vec[0], from_to_vec[1],
                                                multi_from_to_vec[0].to_string(), multi_from_to_vec[1].to_string()));
        }
    }
    return relations_return;
}


fn decode_class_diagram(given_input: String) -> Option<ClassDiagram>{

    let input_regex = Regex::new(r"(.*\|.*)?").unwrap();
    let input = given_input.to_string();
    let mut class_list = Vec::new();
    let mut errors = "".to_string();
    let mut relation_list = Vec::new();

    if input_regex.is_match(input.as_ref()){
        let class_relation_vec: Vec<String> = input.split(&"|".to_string()).map(|x| x.to_owned()).collect();
        class_list = decode_classes(class_relation_vec[0].to_string()).0;
        errors = decode_classes(class_relation_vec[0].to_string()).1;

        if class_relation_vec.len() > 1 {
            relation_list = decode_relations(class_relation_vec[1].to_string());
        }

        return Some(ClassDiagram::new(class_list, relation_list));
    }

    /*for j in &class_list {
        println!("{:?}", j);

    }
    for i in &relation_list {
        println!("{:?}", i);
    }*/

    //call_class_draw(class_list, relation_list);
    return None;
}

fn decode_actors(actors_str: String) -> Vec<Actor>{
    let actor_regex = Regex::new(r"(\d+:\w+:(\d+)?:(\d+( )?)?)").unwrap();
    let mut actors_return = Vec::new();

    let actors_strings = actors_str.split(",");
    for ac_str in actors_strings {
        if actor_regex.is_match(ac_str.as_ref()){

            let mut ac_uc_str = String::new();
            let actor_components: Vec<String> = ac_str.split(&":".to_string()).map(|x| x.to_owned()).collect();
            let mut actor_use_cases = Vec::new();

            let mut extends_from = None;
            if (actor_components[2].to_string() != ""){
                extends_from = Some(actor_components[2].to_string().parse::<i32>().unwrap());
            }

            ac_uc_str = actor_components[3].to_string();
            let actor_use_cases_str = ac_uc_str.split(" ");
            for ac_to_use_str in actor_use_cases_str {
                actor_use_cases.push(ac_to_use_str.to_string().parse::<i32>().unwrap());
            }

            actors_return.push(Actor::new(actor_components[0].to_string().parse::<i32>().unwrap(), actor_components[1].to_string(),
                                          extends_from, actor_use_cases))
        }
    }
    return actors_return;
}


fn decode_use_cases(use_cases_str: String) -> Vec<UseCase>{
    let use_case_regex = Regex::new(r"((\d+:\w+:(EP)?)(,)?)*").unwrap();
    let mut use_case_return = Vec::new();

    let use_case_strings = use_cases_str.split(",");
    for uc_str in use_case_strings {
        if use_case_regex.is_match(uc_str.as_ref()) {

            let use_case_components: Vec<String> = uc_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            use_case_return.push(UseCase::new(use_case_components[0].to_string().parse::<i32>().unwrap(),
                                              use_case_components[1].to_string(), use_case_components[2].to_string() != ""));
        }
    }

    return use_case_return;
}

fn decode_use_case_relations(use_cases_relations_str: String) -> Vec<UseCaseRelation>{
    let relation_use_case_regex = Regex::new(r"((E|I):\d+->\d+)").unwrap();
    let mut relation_uc_return = Vec::new();

    let use_cases_realtions_strings = use_cases_relations_str.split(",");
    for rel_uc_str in use_cases_realtions_strings {
        if relation_use_case_regex.is_match(rel_uc_str.as_ref()) {

            let relation_uc_components: Vec<String> = rel_uc_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            let mut relation_type_uc: UseCaseRelationType;
            match relation_uc_components[0].as_ref(){
                "E" => relation_type_uc = UseCaseRelationType::Extends,
                "I" => relation_type_uc = UseCaseRelationType::Include,
                _=> continue,
            }

            let mut from_to_vec: Vec<i32> = Vec::new();
            for f_t_uc in relation_uc_components[1].split("->"){
                from_to_vec.push(f_t_uc.parse::<i32>().unwrap());
            }

            relation_uc_return.push(UseCaseRelation::new(relation_type_uc, from_to_vec[0], from_to_vec[1]));
        }
    }
    return relation_uc_return;
}

pub fn decode_use_case_diagram(diagram_input: String) -> Option<UseCaseDiagramm>{
    let diagram_input_regex = Regex::new(r"(\w+;.*;.*;.*)").unwrap();
    let diagram_input = diagram_input.to_string();

    if diagram_input_regex.is_match(diagram_input.as_ref()){
        let use_case_diagram_comp: Vec<String> = diagram_input.split(&";".to_string()).map(|x| x.to_owned()).collect();
        return Some(UseCaseDiagramm::new(use_case_diagram_comp[0].to_string(), decode_actors(use_case_diagram_comp[1].to_string()),
                                  decode_use_cases(use_case_diagram_comp[2].to_string()), decode_use_case_relations(use_case_diagram_comp[3].to_string())));


    }
    return None;
}


pub fn decode_input(given_input: String) -> i32{

    let diagram_regex = Regex::new(r"((Class|UseCase)~.*)").unwrap();
    let input = given_input.to_string();
    let mut class_diagram_list = Vec::new();
    let mut use_case_diagram_list = Vec::new();

    let diagram_strings = given_input.split("---");
    for dia_str in diagram_strings {
        if diagram_regex.is_match(dia_str.as_ref()){
            let diagram_components: Vec<String> = dia_str.split(&"~".to_string()).map(|x| x.to_owned()).collect();
            if diagram_components[0] == "Class"{
                let class_return = decode_class_diagram(diagram_components[1].to_string());
                match class_return {
                    Some(class_return) => class_diagram_list.push(class_return),
                    None => continue,
                }
            }
            else if diagram_components[0] == "UseCase"{
                let uc_return = decode_use_case_diagram(diagram_components[1].to_string());
                match uc_return {
                    Some(uc_return) => use_case_diagram_list.push(uc_return),
                    None => continue,
                }
            }
        }
    }
    for j in 0..class_diagram_list.len(){
        call_class_draw(&class_diagram_list[j].classes, &class_diagram_list[j].relations, j as i32)
    }


    for i in &use_case_diagram_list {
        println!("{:?}", i);
    }

    return class_diagram_list.len() as i32;
}

fn call_class_draw(class_list: &Vec<Class>, relation_list: &Vec<Relation>, class_number: i32){
    let path_first_part = "res/UML_visual_result";
    let path_ending = ".png";
    let path_str = [path_first_part, path_ending].join(&class_number.to_string());
    let path = Path::new(&path_str);

    let mut image = visuals::erstelle_image();
    for i in class_list {

        let mut klassentyp = "";
        if let ClassType::Class = i.class_type {
            klassentyp = "Class";
        } else if let ClassType::Abstract = i.class_type {
            klassentyp = "Abstract";
        } else {
            klassentyp = "Interface";
        }

        image = visuals::klasse(i.name.as_ref(), klassentyp, image.clone(), path, i.class_id, &i.attributes, &i.methods);

    }

    for j in relation_list {
        let mut pfeiltyp = "";
        if let RelationType::Vererbung = j.relation_type {
            pfeiltyp = "ver";
        } else if let RelationType::Aggregation = j.relation_type {
            pfeiltyp = "agg";
        } else if let RelationType::Komposition = j.relation_type {
            pfeiltyp = "kompo";
        } else if let RelationType::Assoziation = j.relation_type {
            pfeiltyp = "asso";
        } else if let RelationType::GerAssoziation = j.relation_type {
            pfeiltyp = "ge_asso";
        } else if let RelationType::Implementierung = j.relation_type {
            pfeiltyp = "imple";
        } else {
            pfeiltyp = "abh";
        }

        image = visuals::zeichne_pfeil(image.clone(), path, pfeiltyp, j.from, j.to, &j.from_multiplicity, &j.to_multiplicity);
    }
}