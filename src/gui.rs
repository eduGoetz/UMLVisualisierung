extern crate gtk;
use gtk::*;
use std::process;

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
    pub left_pane: Image,
    pub input: TextBuffer,
    pub start_button: Button,
}


impl UmlGUI {
    fn new() -> UmlGUI {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let mut content = Content::new();

        window.set_titlebar(&header.container);
        window.set_title("UML Visualisierung");
        //window.set_wmclass("app-name", "App name");
        //Window::set_default_icon_name("iconname");
        window.add(&content.container);

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

        let mut left_pane = Image::new();

        let input = TextBuffer::new(None);
        let input_view = TextView::new_with_buffer(&input);
        //input.set_placeholder_text("Eingabe");

        let start_button = Button::new_with_label("Los");
        start_button.get_style_context().map(|c| c.add_class("suggested-action"));


        input_view.set_editable(true);
        input_view.set_wrap_mode(WrapMode::Word);

        let input_scrolled = ScrolledWindow::new(None, None);
        input_scrolled.add(&input_view);

        right_pane.set_border_width(5);
        right_pane.pack_start(&input_scrolled, true, true, 0);
        right_pane.pack_start(&start_button, false, true, 0);

        container.pack1(&left_pane, true, true);
        container.pack2(&right_pane, true, true);

        Content { container, left_pane, input, start_button }

    }

    fn refresh_image(&mut self){
        self.left_pane = Image::new_from_file("res/UML_visual_result.png");
    }
}

pub fn gui_main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let mut gui = UmlGUI::new();
    {
        let input = gui.content.input.clone();
        let mut left_pane = gui.content.left_pane.clone();
        //let (class_list, relation_list) = decoder::decode_input(input.get_text().unwrap());

        gui.content.start_button.connect_clicked(move |_| {
            let (class_list, relation_list) = decoder::decode_input(get_current_input(&input));
            call_class_draw(class_list);
            gui.content.refresh_image();
        });
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