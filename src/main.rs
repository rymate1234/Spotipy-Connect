extern crate gtk;
extern crate librespot;

mod spotify {
    use gtk;
    use gtk::prelude::*;
    use gtk::{Builder, Button, MessageDialog, Window};

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }

        let glade_src = include_str!("ui.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: Window = builder.get_object("mainWindow").unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();

        gtk::main();
    }
}

fn main() {
    spotify::main()
}
