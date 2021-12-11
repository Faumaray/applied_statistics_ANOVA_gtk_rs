#![windows_subsystem = "windows"]
mod data;
use data::dataTable::*;
use gtk::cairo::Surface;
use gtk::prelude::*;
use gtk::Application;
use gtk::{cairo::Context, glib::WeakRef};
use gtk::{cairo::ImageSurface, glib::clone};
use gtk::{gio, glib::clone::Downgrade, glib::clone::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::{fs::File, io::Read};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.Faumaray.applied_statistics_ANOVA_gtk_rs"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let window = Rc::new(gtk::ApplicationWindow::new(application));
    window.set_title(Some("ANOVA"));
    window.set_default_size(1280, 720);
    window.set_resizable(false);
    let main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    main_box.set_margin_top(15);
    main_box.set_margin_bottom(15);
    main_box.set_margin_start(15);
    main_box.set_margin_end(15);
    let right_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let sub_right_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let teory_image_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let matrix_controls: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let calculation_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let mut flow_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .selection_mode(gtk::SelectionMode::None)
        .build();

    let scrolled_window = gtk::ScrolledWindow::builder() // Disable horizontal scrolling
        .min_content_width(200)
        .min_content_height(200)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .width_request(800)
        .height_request(200)
        .has_frame(true)
        .visible(true)
        .child(&flow_box)
        .build();
    let button_show = gtk::Button::builder()
        .label("Показать поля для ввода")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(200)
        .height_request(50)
        .visible(true)
        .build();
    let matrix_count_buffer: gtk::EntryBuffer = gtk::EntryBuffer::new(Option::None);
    let matrix_count = gtk::Entry::builder()
        .buffer(&matrix_count_buffer)
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("Кол-во значений в переменной")
        .height_request(20)
        .width_request(150)
        .build();
    let variables_count_buffer: gtk::EntryBuffer = gtk::EntryBuffer::new(Option::None);
    let variables_count = gtk::Entry::builder()
        .buffer(&variables_count_buffer)
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("Кол-во переменных")
        .height_request(20)
        .width_request(150)
        .build();
    let alfa_buffer: gtk::EntryBuffer = gtk::EntryBuffer::new(Option::None);
    let alfa_count = gtk::Entry::builder()
        .buffer(&alfa_buffer)
        .input_purpose(gtk::InputPurpose::Digits)
        .placeholder_text("Кол-во переменных")
        .height_request(20)
        .width_request(150)
        .build();
    let one_way_cols = gtk::Button::builder()
        .label("Однофакторный по столбцам")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(200)
        .height_request(50)
        .visible(true)
        .build();
    let one_way_rows = gtk::Button::builder()
        .label("Однофакторный по строкам")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(200)
        .height_request(50)
        .visible(true)
        .build();
    let two_way = gtk::Button::builder()
        .label("Двуфакторный")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(200)
        .height_request(50)
        .visible(true)
        .build();
    let theory = gtk::Button::builder()
        .label("Теория")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(410)
        .height_request(100)
        .visible(true)
        .build();
    let crab = gtk::Image::builder().file(".\\static\\crab.png").build();
    crab.set_icon_size(gtk::IconSize::Large);
    crab.set_width_request(400);
    crab.set_height_request(400);
    main_box.append(&scrolled_window);
    matrix_controls.append(&button_show);
    matrix_controls.append(&matrix_count);
    matrix_controls.append(&variables_count);
    calculation_box.append(&alfa_count);
    calculation_box.append(&one_way_cols);
    calculation_box.append(&one_way_rows);
    calculation_box.append(&two_way);
    sub_right_box.append(&matrix_controls);
    sub_right_box.append(&calculation_box);
    teory_image_box.append(&theory);
    teory_image_box.append(&crab);
    right_box.append(&sub_right_box);
    right_box.append(&teory_image_box);
    main_box.append(&right_box);

    let mut entrys_list = Rc::new(RefCell::new(matrix {
        entrys: Vec::<Vec<gtk::Entry>>::new(),
    }));
    let s_for_init = Rc::clone(&entrys_list);
    let s_for_rows = Rc::clone(&entrys_list);
    let s_for_cols = Rc::clone(&entrys_list);
    let s_for_two = Rc::clone(&entrys_list);
    button_show.connect_clicked(clone!(@strong flow_box => move |_| {
        let variables_count: usize = variables_count_buffer.text().parse().unwrap();
        let matrix_count: usize = matrix_count_buffer.text().parse().unwrap();
        if s_for_init.borrow().entrys.len() != 0 {
            for row in &s_for_init.borrow().entrys {
                for ent in row
                {
                    flow_box.remove(&ent.parent().unwrap());
                }

            }
            s_for_init.borrow_mut().entrys = Vec::<Vec<gtk::Entry>>::new();
        }
        flow_box.set_max_children_per_line(variables_count as u32);
        flow_box.set_min_children_per_line(variables_count as u32);
        for _i in 0..variables_count {
            let mut tmp = Vec::new();
            for _j in 0..matrix_count {
                let entry = gtk::Entry::builder().input_purpose(gtk::InputPurpose::Digits).build();
                flow_box.insert(&entry, -1);
                tmp.push(entry);

            }
            tmp.shrink_to_fit();
            s_for_init.borrow_mut().entrys.push(tmp);
        }
    }));
    one_way_cols.connect_clicked(clone!(@strong application =>move |_| {
        let mut vars: Vec<Variable> = Vec::new();
        for i in 0..s_for_cols.borrow().entrys.len() as usize {
            for j in
                0..s_for_cols.borrow().entrys[0].len()
            {
                let mut var: Vec<f64> = Vec::new();
                for k in 0..s_for_cols.borrow().entrys.len() {
                    var.push(s_for_cols.borrow().entrys[k][j].text().parse().unwrap());
                }
                var.shrink_to_fit();
                vars.push(Variable::new(var));
            }
            break;
        }
        let table = DataTable::new(vars);
        create_one_way_window(&application, &table, true, 0.95);
    }));
    one_way_rows.connect_clicked(clone!(@strong application =>move |_| {
        let mut vars: Vec<Variable> = Vec::new();
        for i in 0..s_for_rows.borrow().entrys.len() as usize {
            for j in
                0..s_for_rows.borrow().entrys[0].len()
            {
                let mut var: Vec<f64> = Vec::new();
                for k in 0..s_for_rows.borrow().entrys.len() {
                    var.push(s_for_rows.borrow().entrys[k][j].text().parse().unwrap());
                }
                var.shrink_to_fit();
                vars.push(Variable::new(var));
            }
            break;
        }
        let table = DataTable::new(vars);
        create_one_way_window(&application, &table, false, 0.95);
    }));
    two_way.connect_clicked(clone!(@strong application =>move |_| {
        let mut vars: Vec<Variable> = Vec::new();
        for i in 0..s_for_two.borrow().entrys.len() as usize {
            for j in
                0..s_for_two.borrow().entrys[0].len()
            {
                let mut var: Vec<f64> = Vec::new();
                for k in 0..s_for_two.borrow().entrys.len() {
                    var.push(s_for_two.borrow().entrys[k][j].text().parse().unwrap());
                }
                var.shrink_to_fit();
                vars.push(Variable::new(var));
            }
            break;
        }
        let table = DataTable::new(vars);
        create_two_way_window(&application, &table, 0.95);
    }));
    window.set_child(Some(&main_box));
    window.show();
}
struct matrix {
    entrys: Vec<Vec<gtk::Entry>>,
}
fn create_one_way_window(
    application: &gtk::Application,
    table: &DataTable,
    by_columns: bool,
    alpha: f64,
) {
    let Result = table.one_way(alpha, by_columns);
    let window = gtk::Window::builder()
        .application(application)
        .focus_on_click(true)
        .sensitive(true)
        .resizable(false)
        .startup_id("One way")
        .modal(false)
        .build();

    window.set_title(Some("One way ANOVA"));
    window.set_default_size(400, 200);

    window.show();
}
fn create_two_way_window(application: &gtk::Application, table: &DataTable, alpha: f64) {
    let Result = table.two_way_without_reps(alpha);
    let window = gtk::Window::builder()
        .application(application)
        .focus_on_click(true)
        .sensitive(true)
        .resizable(false)
        .startup_id("Two way")
        .modal(false)
        .build();
    window.set_title(Some("One way ANOVA"));
    window.set_default_size(400, 200);

    window.show();
}
