extern crate regex;
use regex::Regex;

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


pub struct Attribute {
    access_modifier: AccessModifier,
    data_type: String,
    is_final: bool,
    is_static: bool,
    name: String,
}

impl Attribute {
    fn new(access_modifier: AccessModifier, data_type: String, is_final: bool, is_static: bool, name: String) -> Attribute {
        Attribute { access_modifier: access_modifier, data_type: data_type, is_final: is_final, is_static: is_static, name: name}
    }
}


pub struct Parameter {
    data_type: String,
    name: String,
}

impl Parameter {
    fn new(data_type: String, name: String) -> Parameter {
        Parameter { data_type: data_type, name: name}
    }
}


pub struct Method {
    access_modifier: AccessModifier,
    is_final: bool,
    is_static: bool,
    return_type: String,
    name: String,
    parameters: Vec<Parameter>,
}

impl Method{
    fn new(access_modifier: AccessModifier, is_final: bool, is_static: bool, return_type: String, name: String, parameters: Vec<Parameter>) -> Method{
        Method { access_modifier: access_modifier, is_final: is_final, is_static: is_static, return_type: return_type, name: name, parameters: parameters}
    }
}


pub struct Relation {
    relation_type: String,
    from: i32,
    to: i32,
    from_multiplicity: String,
    to_multiplicity: String,
}

impl Relation{
    fn new(relation_type: String, from: i32, to: i32, from_multiplicity: String, to_multiplicity: String) -> Relation{
        Relation { relation_type: relation_type, from: from, to: to, from_multiplicity: from_multiplicity, to_multiplicity: to_multiplicity}
    }
}

enum ClassType{
    Class,
    Interface,
}

enum AccessModifier{
    public,
    private,
    protected,
}

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

    let mut class_list_return: Vec<Class>;
    //let classes_regex = Regex::new(r"\d+;(K|I);(\w+);(\w+:\w+(,?))*;(\w+\(\):\w+(,?))*").unwrap();
    let mut classes_strings = classes_str.split(&"\\".to_string());
    for cla_str in classes_strings {
        if (classes_regex.is_match(cla_str.as_ref())){
            let class_components_vec: Vec<String> = cla_str.split(&";".to_string()).map(|x| x.to_owned()).collect();

            class_list_return.push(Class::new(class_components_vec[0].parse::<i32>().unwrap(), ClassType::class_components_vec[1],
                                              class_components_vec[2], decode_attributes(class_components_vec[3]),
                                              decode_methods(class_components_vec[4])));
        }
    }

    return class_list_return;
}

fn decode_attributes(attributes_str: String) -> Vec<Attribute>{
    let attribute_regex = Regex::new(r"((public|private|protected):(static)?:(final)?:(\w+):(\w+)(,?))*").unwrap();
    let mut class_attributes_return: Vec<Attribute>;

    let mut attributes_strings = attributes_str.split(&",".to_string());
    for attr_str in attributes_strings {
        if (attribute_regex.is_match(attr_str.as_ref())){
            let attribute_components: Vec<String> = attr_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            class_attributes_return.push(Attribute::new(AccessModifier::attribute_components[0],
                                                        attribute_components[1], attribute_components[2] != "",
                                                        attribute_components[3] != "", attribute_components[4]))
        }
    }

    return class_attributes_return;
}


fn decode_methods(methods_str: String) -> Vec<Method>{
    let method_regex = Regex::new(r"((public|private|protected):(static)?:(final)?:(\w+):(\w+))").unwrap();
    let mut class_methods_return: Vec<Method>;

    let mut method_strings = methods_str.split(&",".to_string());
    for met_str in method_strings {
        if (method_regex.is_match(met_str.as_ref())){
            let method_components: Vec<String> = met_str.split(&":".to_string()).map(|x| x.to_owned()).collect();

            class_methods_return.push(Method::new(AccessModifier::method_components[0], method_components[1] != "",
                                                  method_components[2 != ""], method_components[3],
                                                  method_components[4], decode_parameters(method_components[5])));
        }
    }

    return class_methods_return;
}


fn decode_parameters(param_str: String) -> Vec<Parameter>{
    let param_regex = Regex::new(r"(w+)=(w+)").unwrap();
    let mut class_methods_param_return: Vec<Parameter>;

    let mut param_strings = param_str.split(&" ".to_string());
    for par_str in param_strings {
        if (param_regex.is_match(par_str.as_ref())){
            let single_params: Vec<String> = par_str.split(&"=".to_string()).map(|x| x.to_owned()).collect();

            class_methods_param_return.push(Parameter::new(single_params[0], single_params[1]));
        }
    }

    return class_methods_param_return;
}

pub fn decode_input(){

    let input_regex = Regex::new(r"(.*\|.*)?").unwrap();
    let input = "1;Class;Main;public:static:final:int:number;private:static:void:getNumber:int=number String=wort".to_string();
    let mut class_list = Vec::new();

    if input_regex.is_match(input.as_ref()){
        println!("Found match? {}", input_regex.is_match(input.as_ref()));

        let class_relation_vec: Vec<String> = input.split(&"|".to_string()).map(|x| x.to_owned()).collect();
        class_list = decode_classes(class_relation_vec[0]);

    }

    print_classes(class_list);
}

fn print_classes(classes: Vec<Class>){
    for i in &classes {
        println!("{:?}", i);
    }
}
