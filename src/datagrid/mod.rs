mod imp;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct ApplicationRow(ObjectSubclass<imp::ApplicationRow>)
        @extends gtk::Widget, gtk::Box;
}

impl Default for ApplicationRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ApplicationRow {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ApplicationRow")
    }

    pub fn set_app_info(&self, app_info: &gio::AppInfo) {
        let self_ = imp::ApplicationRow::from_instance(self);
        self_.value.set_text(&app_info.name());
    }
}
