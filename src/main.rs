mod data;
use data::dataTable::*;
use gtk::cairo::Context;
use gtk::cairo::Format;
use gtk::cairo::ImageSurface;
use gtk::cairo::Surface;
use gtk::prelude::*;
use gtk::Application;
use gtk::{gio, glib::clone};
use std::{cell::RefCell, rc::Rc};
use std::{fs::File, io::Read};

fn main() {
    // Тестовые величины
    let var1 = Variable::new(vec![
        1.82307, 1.337521, 0.709904, 0.706639, 1.37292, 0.755022,
    ]);
    let var2 = Variable::new(vec![
        1.763353, 1.404699, -0.15568, 0.974917, 0.739462, 0.841902,
    ]);
    let var3 = Variable::new(vec![
        2.057529, 1.081195, 0.86748, 0.66569, 1.010997, 1.793682,
    ]);
    let var4 = Variable::new(vec![
        1.018306, 0.393067, 1.573685, 0.171241, 1.32679, 1.305396,
    ]);
    let var5 = Variable::new(vec![
        0.80303, 1.252988, 1.944237, 0.362799, 0.650761, 0.37149,
    ]);
    let table = DataTable::new(vec![var1, var2, var3, var4, var5]);
    //Тестовая зона
    let one_way_columns = table.one_way(0.05, true);
    let one_way_rows = table.one_way(0.05, false);
    let two_way = table.two_way_without_reps(0.05);
    //Конец тестовой зоны
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
    window.set_resizable(true);
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
        entrys: Vec::<gtk::Entry>::new(),
    }));
    button_show.connect_clicked(move |_| {
        let variables_count: usize = variables_count_buffer.text().parse().unwrap();
        let matrix_count: usize = matrix_count_buffer.text().parse().unwrap();
        if entrys_list.borrow().entrys.len() != 0 {
            for entry in &entrys_list.borrow().entrys {
                flow_box.remove(&entry.parent().unwrap());
            }
            entrys_list.borrow_mut().entrys = Vec::<gtk::Entry>::new();
        }
        flow_box.set_max_children_per_line(variables_count as u32);
        flow_box.set_min_children_per_line(variables_count as u32);
        for _i in 0..variables_count {
            for _j in 0..matrix_count {
                let entry = gtk::Entry::new();
                flow_box.insert(&entry, -1);
                entrys_list.borrow_mut().entrys.push(entry);
            }
        }
    });
    window.set_child(Some(&main_box));
    window.show();
}
struct matrix {
    entrys: Vec<gtk::Entry>,
}
