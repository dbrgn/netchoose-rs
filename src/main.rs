extern crate gtk;

mod netctl;

use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use gtk::{Box, Orientation};
use gtk::{ListStore, TreeView, TreeViewColumn, CellRendererText};
use gtk::{Toolbar, ToolButton};

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

fn create_toolbar() -> Toolbar {
    let toolbar = Toolbar::new();
    let refresh = ToolButton::new_from_stock("gtk-refresh");
    ToolItemExt::set_tooltip_text(&refresh, "Reload the list of profiles");
    toolbar.insert(&refresh, -1);
    toolbar
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

    // Create vertical box view
    let v_box = Box::new(Orientation::Vertical, 0);

    // Create toolbar
    let toolbar = create_toolbar();

    // Create tree view
    let list = create_and_setup_view();

    // Attach a model to tree view
    let model = create_and_fill_model();
    list.set_model(Some(&model));

    // Handle double click
    list.connect_row_activated(|tree_view, _, _| {
        let selection = tree_view.get_selection();
        if let Some((model, iter)) = selection.get_selected() {
            let name = model.get_value(&iter, 1)
                            .get::<String>()
                            .expect("Could not get profile name");
            netctl::switch_to_profile(&name);
        };
    });

    // Show all widgets
    v_box.pack_start(&toolbar, false, false, 0);
    v_box.pack_start(&list, true, true, 0);
    window.add(&v_box);
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
