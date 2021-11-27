
mod data;
use data::DataTable::*;
use gtk::prelude::*;
use gtk::glib::clone;
use gtk::glib::signal::Inhibit;
use gtk::{Application, TextBuffer, TextView};
use std::rc::Rc;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.Faumaray.applied_statistics_ANOVA_gtk_rs"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(application: &Application) {
    let mut window = Rc::new(gtk::ApplicationWindow::new(application));
    window.set_title(Some("ANOVA"));
    window.set_default_size(900, 720);
    window.set_resizable(false);
    window.show();
}