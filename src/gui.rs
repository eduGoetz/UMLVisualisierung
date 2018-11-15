extern crate gtk;
use gtk::*;
use std::process;

use decoder;

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
    pub left_pane: TextBuffer,
    pub input: Entry,
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

        let left_pane = TextBuffer::new(None);
        let left_pane_view = TextView::new_with_buffer(&left_pane);

        let input = Entry::new();
        input.set_placeholder_text("Eingabe");

        let start_button = Button::new_with_label("Los");
        start_button.get_style_context().map(|c| c.add_class("suggested-action"));

        left_pane_view.set_editable(false);
        left_pane_view.set_wrap_mode(WrapMode::Word);

        let left_pane_scrolled = ScrolledWindow::new(None, None);
        left_pane_scrolled.add(&left_pane_view);

        right_pane.set_border_width(5);
        right_pane.pack_start(&input, false, true, 0);
        right_pane.pack_start(&start_button, false, true, 0);


        //container.pack1(&left_pane, true, true);
        container.pack1(&left_pane_scrolled, true, true);
        container.pack2(&right_pane, true, true);

        Content { container, left_pane, input, start_button }

    }
}

pub fn gui_main() {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK Application");
        process::exit(1);
    }

    let gui = UmlGUI::new();

    {
        let input = gui.content.input.clone();
        let left_pane = gui.content.left_pane.clone();
        let (class_list, relation_list) = decoder::decode_input(input.get_text().unwrap());

        gui.content.start_button.connect_clicked(move |_| {
            for i in class_list {
                left_pane.set_text((get_current_buffer(&left_pane).as_ref()));
                left_pane.set_text(i.to_string().as_ref());
            }
        });
    }

    gui.window.show_all();

    gtk::main();
}

fn get_current_buffer(buffer: &TextBuffer) -> String {
    let start = buffer.get_start_iter();
    let end = buffer.get_end_iter();
    buffer.get_text(&start, &end, true).to_string();
}