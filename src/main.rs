extern crate gtk;

mod netctl;

use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use gtk::{ListStore, TreeView, TreeViewColumn, CellRendererText};

fn create_and_fill_model() -> ListStore {
    let model = ListStore::new(&[String::static_type(), String::static_type()]);
    let profiles = netctl::get_profiles();
    for profile in profiles.iter() {
        let active = if profile.active { "*" } else { "" };
        let name = &profile.name;
        model.insert_with_values(None, &[0, 1], &[&active, name]);
    }
    model
}

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn create_and_setup_view() -> TreeView {
    // Creating the tree view.
    let tree = TreeView::new();

    // Hide headers
    tree.set_headers_visible(false);

    // Create the columns inside the view
    append_column(&tree, 0);
    append_column(&tree, 1);

    tree
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Init window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Netchoose");
    window.set_default_size(240, 320);
    window.set_position(WindowPosition::Center);

    // Create list view
    let list = create_and_setup_view();
    let model = create_and_fill_model();
    list.set_model(Some(&model));
    window.add(&list);

    // Show all widgets
    window.show_all();

    // Window closing event
    window.connect_delete_event(|_, _| {
        // Stop the main loop
        gtk::main_quit();

        // Let the default handler destroy the window.
        Inhibit(false)
    });

    // Start the main event loop
    gtk::main();
}
