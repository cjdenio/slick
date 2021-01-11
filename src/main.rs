use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button, StackSidebar, Stack};

fn main() {
    let application = Application::new(Some("me.calebdenio.slick"), Default::default()).expect("Failed to launch application :(");


    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("title!");

        let button = Button::new();
        button.set_label("Click me!");

        button.connect_clicked(|b| {
            println!("hi");
            b.set_label("clicked");
        });

        let sidebar = StackSidebar::new();
        let stack = Stack::new();
        sidebar.set_stack(&stack);

        window.add(&button);
        window.add(&sidebar);

        window.show_all();
    });

    application.run(&[]);
}
