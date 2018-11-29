extern crate gtk;
use gtk::*;
use std::process;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;

use decoder;
use visuals;

pub struct UmlGUI {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar
}

pub struct Content {
    pub container: Paned,
    pub left_pane: Rc<RefCell<Image>>,
    pub input: TextBuffer,
    pub class_template_button: Button,
    pub relation_template_button: Button,
    pub noti_label: Label,
    pub start_button: Button,
}


impl UmlGUI {
    fn new() -> UmlGUI {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        window.set_titlebar(&header.container);
        window.set_title("UML Visualisierung");
        //window.set_wmclass("app-name", "App name");
        //Window::set_default_icon_name("iconname");
        window.add(&content.container);
        window.maximize();

        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });

        // Return our main application state
        UmlGUI { window, header, content }
    }

    fn set_new_noti(&mut self, notific: String){
        let label_content = self.content.noti_label.get_text().unwrap();
        self.content.noti_label.set_label([label_content, notific].join("\n").as_ref());
    }
}


impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        container.set_title("UML Visualisierung");
        container.set_show_close_button(true);

        return Header { container }
    }
}


impl Content {
    fn new() -> Content {
        let container = Paned::new(Orientation::Horizontal);
        let right_pane = Box::new(Orientation::Vertical, 3);

        let left_pane: Rc<RefCell<Image>> = Rc::new(RefCell::new(Image::new()));

        let input = TextBuffer::new(None);
        let input_view = TextView::new_with_buffer(&input);
        //input.set_placeholder_text("Eingabe");

        let class_template_button = Button::new_with_label("Neues Klasse-Template");
        let input_clone = input.clone();
        class_template_button.connect_clicked(move |_| {
            //let template = "ID;Class oder Interface;Name;Zugriffsmodifikator:static:final:int:number:,;Zugriffsmodifikator:static:final:void:name:Parametertyp=Parametername Parametertyp2=Parametername2/".to_owned();
            let template = "ID;Typ;Name;Sichtbarkeit:static:final:Datentyp:name,;Sichtbarkeit:static:final:Rückgabetyp:Name:Paramtyp=Paramname Paramtyp=Paramname2/".to_owned();
            input_clone.set_text([get_current_input(&input_clone), template].join("").as_ref());
        });

        let relation_template_button = Button::new_with_label("Neues Relation-Template");
        let input_clone = input.clone();
        relation_template_button.connect_clicked(move |_ | {
            let template = "Typ;IDvon->IDzu;".to_owned();
            input_clone.set_text([get_current_input(&input_clone), template].join("").as_ref());
        });

        let start_button = Button::new_with_label("Los");
        start_button.get_style_context().map(|c| c.add_class("suggested-action"));

        let noti_label = Label::new("i - Relationen kommen zuletzt und werden mit einem '|' von den Klassen getrennt.");

        let input_clone = input.clone();
        let left_pane_clone = left_pane.clone();
        let label_clone = noti_label.clone();
        start_button.connect_clicked(move |start_button| {
            let (class_list, relation_list, errors) = decoder::decode_input(get_current_input(&input_clone));
            label_clone.set_text(errors.as_ref());
            call_class_draw(class_list, relation_list);

            //*left_pane_clone.borrow_mut() = Image::new_from_file("res/1540040897129.png");
            Image::set_from_file(&*left_pane_clone.borrow_mut(), "res/UML_visual_result.png");
            //Content::refresh_image(&mut Self);
        });

        input_view.set_editable(true);
        input_view.set_wrap_mode(WrapMode::Char);

        let input_scrolled = ScrolledWindow::new(None, None);
        input_scrolled.add(&input_view);

        right_pane.set_border_width(5);
        right_pane.pack_start(&class_template_button, false, false, 0);
        right_pane.pack_start(&relation_template_button, false, false, 0);
        right_pane.pack_start(&input_scrolled, true, true, 0);
        right_pane.pack_start(&noti_label, false, true, 0);
        right_pane.pack_start(&start_button, false, true, 0);

        //container.add(&*left_pane_clone.borrow_mut());
        container.pack1(&*left_pane.borrow_mut(), true, true);
        container.pack2(&right_pane, true, true);

        //let left_pane = left_pane.into_inner();

        Content { container, left_pane, input, class_template_button, relation_template_button, noti_label, start_button }

    }

    /*fn refresh_image(&mut self){
        self.left_pane = Image::new_from_file("res/UML_visual_result.png");
    }*/
}


pub fn gui_main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let mut gui = UmlGUI::new();
    {
        /*let input = gui.content.input.clone();
        let mut left_pane = gui.content.left_pane.clone();
        //let (class_list, relation_list) = decoder::decode_input(input.get_text().unwrap());

        gui.content.start_button.connect_clicked(move |_| {
            let (class_list, relation_list) = decoder::decode_input(get_current_input(&input));
            call_class_draw(class_list);

            left_pane = Image::new_from_file("res/UML_visual_result.png");
            //gui.content.refresh_image();
        });*/
    }

    gui.window.show_all();

    gtk::main();
}

fn get_current_input(buffer: &TextBuffer) -> String {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    return buffer.get_text(&start, &end, true).unwrap();
}


fn call_class_draw(class_list: Vec<decoder::Class>, relation_list: Vec<decoder::Relation>){
    let path = Path::new("res/UML_visual_result.png");
    let mut image = visuals::erstelle_image();
    for i in &class_list {

        let mut klassentyp = "";
        if let decoder::ClassType::Class = i.class_type {
            klassentyp = "Class";
        } else if let decoder::ClassType::Abstract = i.class_type {
            klassentyp = "Abstract";
        } else {
            klassentyp = "Interface";
        }

        //let mut attr_vec= Vec::new();
        //for attr in i.attributes{

        //}

        let mut meth_vec= Vec::new();
        //for meth in i.methods{
            meth_vec.push("fdfd");
        //}

        image = visuals::klasse(i.name.as_ref(), klassentyp, image.clone(), path, i.class_id, &i.attributes, &i.methods);

    }

    for j in &relation_list {
        let mut pfeiltyp = "";
        if let decoder::RelationType::Vererbung = j.relation_type {
            pfeiltyp = "ver";
        } else if let decoder::RelationType::Aggregation = j.relation_type {
            pfeiltyp = "agg";
        } else if let decoder::RelationType::Komposition = j.relation_type {
            pfeiltyp = "kompo";
        } else if let decoder::RelationType::Assoziation = j.relation_type {
            pfeiltyp = "asso";
        } else if let decoder::RelationType::GerAssoziation = j.relation_type {
            pfeiltyp = "ge_asso";
        } else if let decoder::RelationType::Implementierung = j.relation_type {
            pfeiltyp = "imple";
        } else {
            pfeiltyp = "abh";
        }

        image = visuals::zeichne_pfeil(image.clone(), path, pfeiltyp, j.from, j.to);
    }
}