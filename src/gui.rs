extern crate gtk;
use gtk::*;
use std::process;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

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
            let template = ";;;:::::,;:::::= =/".to_owned();
            input_clone.set_text([get_current_input(&input_clone), template].join("\n").as_ref());
        });


        let relation_template_button = Button::new_with_label("Neues Relation-Template");

        let start_button = Button::new_with_label("Los");
        start_button.get_style_context().map(|c| c.add_class("suggested-action"));

        let input_clone = input.clone();
        let left_pane_clone = left_pane.clone();
        start_button.connect_clicked(move |start_button| {
            let (class_list, relation_list) = decoder::decode_input(get_current_input(&input_clone));
            call_class_draw(class_list);
            println!("FDFDF");

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
        right_pane.pack_start(&start_button, false, true, 0);

        //container.add(&*left_pane_clone.borrow_mut());
        container.pack1(&*left_pane.borrow_mut(), true, true);
        container.pack2(&right_pane, true, true);

        //let left_pane = left_pane.into_inner();

        Content { container, left_pane, input, class_template_button, relation_template_button, start_button }

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

fn call_class_draw(class_list: Vec<decoder::Class>){
        visuals::klasse("klassenname","agg","attributwert","methodenwert");
}