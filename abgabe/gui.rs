extern crate gtk;

use gtk::*;
use gtk::prelude::*;
use std::process;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use decoder;
use visuals;

use webbrowser;

pub struct UmlGUI {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct Header {
    pub container: HeaderBar,
    pub open_file: Button,
    pub open_doc: Button,
}

pub struct Content {
    pub container: Paned,
    pub notebook: Rc<RefCell<Notebook>>,
    pub input: TextBuffer,
    pub class_template_button: Button,
    pub relation_template_button: Button,
    pub noti_label: Label,
    pub start_button: Button,
}

#[derive(Clone)]
pub struct Notebook {
    notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>,
}

impl Notebook {
    fn new() -> Notebook {
        Notebook {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new(),
        }
    }


    fn create_tab(&mut self, title: &str, image_path: String) {
        let label = gtk::Label::new(title);
        let tab = gtk::Box::new(Orientation::Horizontal, 0);

        tab.pack_start(&label, false, false, 0);
        tab.show_all();

        let img = Image::new_from_file(&image_path);

        let scoll_pane = ScrolledWindow::new(None, None);
        scoll_pane.add(&img);

        self.notebook.append_page(&scoll_pane, Some(&tab));

        self.tabs.push(tab);
    }
}


impl UmlGUI {
    fn new() -> UmlGUI {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        let input_clone = content.input.clone();
        header.open_file.connect_clicked(move |_| {
            let dialog = FileChooserDialog::new(Some("Datei öffnen"), Some(&Window::new(WindowType::Popup)), FileChooserAction::Open);

            dialog.add_button("Abbrechen", ResponseType::Cancel.into());
            dialog.add_button("Öffnen", ResponseType::Accept.into());

            let button_select = dialog.run();
            if button_select == ResponseType::Accept.into() {
                let file_path = dialog.get_filename().unwrap();
                let mut file = File::open(file_path).unwrap();
                let mut file_content = String::new();
                file.read_to_string(&mut file_content);
                input_clone.set_text(&file_content);
                dialog.close();
            } else if button_select == ResponseType::Cancel.into() {
                dialog.close();
            }
        });

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

        UmlGUI { window, header, content }
    }

    fn set_new_noti(&mut self, notific: String) {
        let label_content = self.content.noti_label.get_text().unwrap();
        self.content.noti_label.set_label([label_content, notific].join("\n").as_ref());
    }
}


impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();

        let open_file = Button::new_with_label("Datei");

        let open_doc = Button::new_with_label("Dokumentation");
        open_doc.connect_clicked(move |_| {
            webbrowser::open("https://github.com/eduGoetz/UMLVisualisierung/blob/master/README.md");
        });

        container.set_title("UML Visualisierung");
        container.set_show_close_button(true);
        container.add(&open_file);
        container.add(&open_doc);

        return Header { container, open_file, open_doc };
    }
}


impl Content {
    fn new() -> Content {
        let container = Paned::new(Orientation::Horizontal);
        let right_pane = Box::new(Orientation::Vertical, 3);

        let left_pane_image: Rc<RefCell<Image>> = Rc::new(RefCell::new(Image::new()));

        let mut notebook: Rc<RefCell<Notebook>> = Rc::new(RefCell::new(Notebook::new()));

        let input = TextBuffer::new(None);
        let input_view = TextView::new_with_buffer(&input);

        let class_template_button = Button::new_with_label("Neues Klasse-Template");
        let input_clone = input.clone();
        class_template_button.connect_clicked(move |_| {
            let template = "ID;Typ;Name;Sichtbarkeit:static:final:Datentyp:name,;Sichtbarkeit:static:final:Rückgabetyp:Name:Paramtyp=Paramname Paramtyp=Paramname2/".to_owned();
            input_clone.set_text([get_current_input(&input_clone), template].join("").as_ref());
        });

        let relation_template_button = Button::new_with_label("Neues Relation-Template");
        let input_clone = input.clone();
        relation_template_button.connect_clicked(move |_| {
            let template = "Typ;IDvon->IDzu;".to_owned();
            input_clone.set_text([get_current_input(&input_clone), template].join("").as_ref());
        });

        let start_button = Button::new_with_label("Los");
        start_button.get_style_context().map(|c| c.add_class("suggested-action"));

        let noti_label = Label::new("i - Relationen kommen zuletzt und werden mit einem '|' von den Klassen getrennt.");

        let input_clone = input.clone();
        let left_pane_clone = left_pane_image.clone();
        let label_clone = noti_label.clone();
        let mut notebook_clone = notebook.clone();

        input_view.set_editable(true);
        input_view.set_wrap_mode(WrapMode::Char);

        let input_scrolled = ScrolledWindow::new(None, None);
        input_scrolled.add(&input_view);

        //let left_pane = ScrolledWindow::new(None, None);
        //left_pane.add(&notebook.borrow_mut().notebook);
        //left_pane.add(&*left_pane_image.borrow_mut());

        right_pane.set_border_width(5);
        right_pane.pack_start(&class_template_button, false, false, 0);
        right_pane.pack_start(&relation_template_button, false, false, 0);
        right_pane.pack_start(&input_scrolled, true, true, 0);
        right_pane.pack_start(&noti_label, false, true, 0);
        right_pane.pack_start(&start_button, false, true, 0);

        container.pack1(&notebook.borrow_mut().notebook, true, true);
        container.pack2(&right_pane, true, true);

        Content { container, notebook, input, class_template_button, relation_template_button, noti_label, start_button }
    }
}


pub fn gui_main() {
    if gtk::init().is_err() {
        eprintln!("Fehler");
        process::exit(1);
    }

    let mut gui = UmlGUI::new();

    let window_clone = gui.window.clone();
    //let notebook = gui.content.notebook;
    let notebook_clone = gui.content.notebook.clone();
    let input_clone = gui.content.input.clone();
    let label_clone = gui.content.noti_label.clone();
    gui.content.start_button.connect_clicked(move |_| {
        //let errors = decoder::decode_input(get_current_input(&input_clone).replace('\n', ""));
        //label_clone.set_text(errors.as_ref());

        let amount_model = decoder::decode_input(get_current_input(&input_clone).replace('\n', ""));
        label_clone.set_label(&amount_model.errors);

        for j in 0..(amount_model.class_amount + amount_model.use_case_amount) as u32 {
            notebook_clone.borrow_mut().notebook.remove_page(Some(0));
        }

        //*left_pane_clone.borrow_mut() = Image::new_from_file("res/1540040897129.png");
        //Image::set_from_file(&*left_pane_clone.borrow_mut(), "res/UML_visual_result.png");
        for i in 0..amount_model.class_amount {
            notebook_clone.borrow_mut().create_tab(&["ClassDiagramm", &i.to_string()].join(""), [&amount_model.class_file_name, ".png"].join(&i.to_string()));
            window_clone.show_all();
        }

        for j in 0..amount_model.use_case_amount {
            notebook_clone.borrow_mut().create_tab(&["UseCaseDiagramm", &j.to_string()].join(""), [&amount_model.use_case_file_name, ".png"].join(&j.to_string()));
            window_clone.show_all();
        }
    });

    gui.window.show_all();
    gtk::main();
}

fn get_current_input(buffer: &TextBuffer) -> String {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    return buffer.get_text(&start, &end, true).unwrap();
}