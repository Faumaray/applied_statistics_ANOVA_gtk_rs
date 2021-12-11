use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::CompositeTemplate;

#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "application_row.ui")]
pub struct ApplicationRow {
    #[template_child]
    pub value: TemplateChild<gtk::Entry>,
}

#[glib::object_subclass]
impl ObjectSubclass for ApplicationRow {
    const NAME: &'static str = "ApplicationRow";
    type Type = super::ApplicationRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ApplicationRow {}
impl WidgetImpl for ApplicationRow {}
impl BoxImpl for ApplicationRow {}
