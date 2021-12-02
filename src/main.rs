
mod data;
use data::DataTable::*;
use gtk::prelude::*;
use gtk::glib::clone;
use gtk::glib::signal::Inhibit;
use gtk::{Application, TextBuffer, TextView};
use std::rc::Rc;

fn main() {
    let var1= Variable::new(vec![1.82307, 1.337521,0.709904, 0.706639, 1.37292,0.755022]);
    let var2 = Variable::new(vec![1.763353,1.404699,-0.15568,0.974917, 0.739462, 0.841902]);
    let var3 = Variable::new(vec![2.057529,1.081195,0.86748,0.66569,1.010997,1.793682]);
    let var4 = Variable::new(vec![1.018306,0.393067,1.573685,0.171241,1.32679,1.305396]);
    let table = DataTable::new(vec![var1,var2,var3,var4]);
    let one_row = one_way(table.clone(), 0.05, false);
    println!("{:#?}",table);
    println!("{:?}",one_row);
       // MSe = SSe/a-1 (а - кол-во строк\столбцов)
        //MSa = SSa/n-a    
    /*let application =
        gtk::Application::new(Some("com.github.Faumaray.applied_statistics_ANOVA_gtk_rs"), Default::default());
    application.connect_activate(build_ui);
    application.run();*/
}
fn build_ui(application: &Application) {
    let mut window = Rc::new(gtk::ApplicationWindow::new(application));
    window.set_title(Some("ANOVA"));
    window.set_default_size(900, 720);
    window.set_resizable(false);
    window.show();
}