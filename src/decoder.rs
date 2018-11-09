extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub struct Class {
    class_id: i32,
    class_type: ClassType,
    name: String,
    attributes: Vec<Attribute>,
    methods: Vec<Method>,
}

impl Class {
    fn new(class_id: i32, class_type: ClassType, name: String, attributes: Vec<Attribute>, methods: Vec<Method>) -> Class {
        Class {class_id: class_id, class_type: class_type, name: name, attributes: attributes, methods: methods}
    }
}

#[derive(Debug)]
pub struct Attribute {
    access_modifier: AccessModifier,
    is_static: bool,
    is_final: bool,
    data_type: String,
    name: String,
}

impl Attribute {
    fn new(access_modifier: AccessModifier, is_static: bool, is_final: bool, data_type: String, name: String) -> Attribute {
        Attribute { access_modifier: access_modifier, is_static: is_static, is_final: is_final, data_type: data_type, name: name}
    }
}

#[derive(Debug)]
pub struct Parameter {
    data_type: String,
    name: String,
}

impl Parameter {
    fn new(data_type: String, name: String) -> Parameter {
        Parameter { data_type: data_type, name: name}
    }
}

#[derive(Debug)]
pub struct Method {
    access_modifier: AccessModifier,
    is_static: bool,
    is_final: bool,
    return_type: String,
    name: String,
    parameters: Vec<Parameter>,
}

impl Method{
    fn new(access_modifier: AccessModifier, is_static: bool, is_final: bool, return_type: String, name: String, parameters: Vec<Parameter>) -> Method{
        Method { access_modifier: access_modifier, is_static: is_static, is_final: is_final, return_type: return_type, name: name, parameters: parameters}
    }
}

#[derive(Debug)]
pub struct Relation {
    relation_type: RelationType,
    from: i32,
    to: i32,
    from_multiplicity: String,
    to_multiplicity: String,
}

impl Relation{
    fn new(relation_type: RelationType, from: i32, to: i32, from_multiplicity: String, to_multiplicity: String) -> Relation{
        Relation { relation_type: relation_type, from: from, to: to, from_multiplicity: from_multiplicity, to_multiplicity: to_multiplicity}
    }
}
#[derive(Debug)]
enum ClassType{
    Class,
    Interface,
}
#[derive(Debug)]
enum AccessModifier{
    Public,
    Private,
    Protected,
}
#[derive(Debug)]
enum RelationType{
    Vererbung,
    Aggregation,
    Komposition,
    Assoziation,
    GerAssoziation,
    Implementierung,
}


fn decode_classes(classes_str: String) -> Vec<Class>{
    let classes_regex = Regex::new(r"(\d+;((Class)|(Interface));(\w+);(.*);(.*)(\\?))+").unwrap();

    let mut class_list_return = Vec::new();
    let classes_strings = classes_str.split("/");
    for cla_str in classes_strings {
        if classes_regex.is_match(cla_str.as_ref()){
            let class_components_vec: Vec<String> = cla_str.split(&";".to_string()).map(|x| x.to_owned()).collect();

            let mut class_type: ClassType;
            match class_components_vec[1].as_ref(){
                "Class" => class_type = ClassType::Class,
                "Interface" => class_type =  ClassType::Interface,
                _ => continue,
            }

            class_list_return.push(Class::new(class_components_vec[0].parse::<i32>().unwrap(), class_type,
                                              class_components_vec[2].to_string(), decode_attributes(class_components_vec[3].to_string()),
                                              decode_methods(class_components_vec[4].to_string())));
        }
    }
    return class_list_return;
}


fn decode_attributes(attributes_str: String) -> Vec<Attribute>{
    let attribute_regex = Regex::new(r"((public|private|protected):(static)?:(final)?:(\w+):(\w+)(,?))*").unwrap();
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
                _ => continue,
            }

            class_attributes_return.push(Attribute::new(attribute_access_modifier,attribute_components[1].to_string() != "",
                                                        attribute_components[2].to_string() != "", attribute_components[3].to_string(),
                                                        attribute_components[4].to_string()))
        }
    }
    return class_attributes_return;
}

//private:static::void:getNumber:int=number String=wort
fn decode_methods(methods_str: String) -> Vec<Method>{
    let method_regex = Regex::new(r"((public|private|protected):(static)?:(final)?:(\w+):(\w+):(.*)?)").unwrap();
    let mut class_methods_return = Vec::new();

    let method_strings = methods_str.split(",");
    for met_str in method_strings {
        if method_regex.is_match(met_str.as_ref()){
            let method_components: Vec<String> = met_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            let mut method_access_modifier: AccessModifier;
            match method_components[0].as_ref(){
                "public" => method_access_modifier = AccessModifier::Public,
                "private" => method_access_modifier =  AccessModifier::Private,
                "protected" => method_access_modifier = AccessModifier::Protected,
                _ => continue,
            }

            class_methods_return.push(Method::new(method_access_modifier, method_components[1].to_string() != "",
                                                  method_components[2].to_string() != "", method_components[3].to_string(),
                                                  method_components[4].to_string(), decode_parameters(method_components[5].to_string())));
        }
    }

    return class_methods_return;
}


fn decode_parameters(param_str: String) -> Vec<Parameter>{
    let param_regex = Regex::new(r"((w+)=(w+)( )?)*").unwrap();
    let mut class_methods_param_return = Vec::new();

    let param_strings = param_str.split(" ");
    for par_str in param_strings {
        if param_regex.is_match(par_str.as_ref()){
            let single_params: Vec<String> = par_str.split(&"=".to_string()).map(|x| x.to_owned()).collect();

            class_methods_param_return.push(Parameter::new(single_params[0].to_string(), single_params[1].to_string()));
        }
    }
    return class_methods_param_return;
}


fn decode_relations(relations_str: String) -> Vec<Relation>{
    let relation_regex = Regex::new(r"((V|A|gA|AG|K|I);\d+->\d+;((\d+|\w+|\*):(\d+|\w+|\*))?(,)?)*").unwrap();
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

pub fn decode_input(){

    let input_regex = Regex::new(r"(.*\|.*)?").unwrap();
    let input =
        "1;Class;Main;public:static::int:number;private:static::void:getNumber:int=number String=wort/2;Class;Main;public:static::int:number;private:static::void:getNumber:int=number String=wort|V;2->1;"
            .to_string();
    let mut class_list = Vec::new();
    let mut relation_list = Vec::new();

    if input_regex.is_match(input.as_ref()){
        let class_relation_vec: Vec<String> = input.split(&"|".to_string()).map(|x| x.to_owned()).collect();
        class_list = decode_classes(class_relation_vec[0].to_string());

        if class_relation_vec.len() > 1 {
            relation_list = decode_relations(class_relation_vec[1].to_string());
        }

    }

    print_classes(class_list);

    for i in relation_list {
        println!("{:?}", i);
    }
}

fn print_classes(classes: Vec<Class>){
    for i in &classes {
        println!("{:?}", i);
    }
}

