#![windows_subsystem = "windows"]
mod data;
use data::dataTable::*;
use execute::shell;
use gtk::glib::clone;
use gtk::prelude::*;
use gtk::Application;
use std::{cell::RefCell, fs::File, io::Read, path::PathBuf, rc::Rc};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.Faumaray.applied_statistics_ANOVA_gtk_rs"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}
static fileList: [&str; 6] = ["theory", "0.001", "0.01", "0.1", "0.05", "0.025"];

fn build_ui(application: &Application) {
    let window = Rc::new(gtk::ApplicationWindow::new(application));
    window.set_title(Some("ANOVA"));
    window.set_default_size(1200, 500);
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
        .height_request(500)
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
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .width_request(410)
        .height_request(100)
        .visible(true)
        .build();
    let name = gtk::Label::builder()
        .use_markup(true)
        .height_request(24)
        .width_request(400)
        .build();
    name.set_markup("<span size='28000'>Дисперсионный анализ</span>");
    let crab = gtk::Label::builder()
        .use_markup(true)
        .wrap(true)
        .label("Инструкция пользования: \n1)Введите размерность матрицы входных значений и нажать кнопку показать\n2)Заполнить матрицу\n3)Выбрать необходимый вид анализа для получения результата\n4)На windows окна могут вести себя странно\n5)При необходимости имеется возможность просмотра теоретической информации")
        .height_request(60)
        .width_request(400)
        .build();
    main_box.append(&scrolled_window);
    matrix_controls.append(&button_show);
    matrix_controls.append(&matrix_count);
    matrix_controls.append(&variables_count);
    calculation_box.append(&one_way_cols);
    calculation_box.append(&one_way_rows);
    calculation_box.append(&two_way);
    sub_right_box.append(&matrix_controls);
    sub_right_box.append(&calculation_box);
    teory_image_box.append(&theory);
    teory_image_box.append(&name);
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
        }
        s_for_init.borrow_mut().entrys = vec![vec![gtk::Entry::new();variables_count];matrix_count];
        flow_box.set_max_children_per_line(variables_count as u32);
        flow_box.set_min_children_per_line(variables_count as u32);
        for i in 0..matrix_count {
            for j in 0..variables_count {
                let entry = gtk::Entry::builder().input_purpose(gtk::InputPurpose::Digits).build();
                flow_box.insert(&entry, -1);
                s_for_init.borrow_mut().entrys[i][j]= entry;

            }
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
                    var.push(s_for_cols.borrow().entrys[k][j].text().replace(",",".").parse().unwrap());
                }
                var.shrink_to_fit();
                vars.push(Variable::new(var));
            }
            break;
        }
        let table = DataTable::new(vars);
        create_one_way_window(&application, &table, true);
    }));
    one_way_rows.connect_clicked(clone!(@strong application =>move |_| {
        let mut vars: Vec<Variable> = Vec::new();
        for i in 0..s_for_rows.borrow().entrys.len() as usize {
            for j in
                0..s_for_rows.borrow().entrys[0].len()
            {
                let mut var: Vec<f64> = Vec::new();
                for k in 0..s_for_rows.borrow().entrys.len() {
                    var.push(s_for_rows.borrow().entrys[k][j].text().replace(",",".").parse().unwrap());
                }
                var.shrink_to_fit();
                vars.push(Variable::new(var));
            }
            break;
        }
        let table = DataTable::new(vars);
        create_one_way_window(&application, &table, false);
    }));
    two_way.connect_clicked(clone!(@strong application =>move |_| {
        let mut vars: Vec<Variable> = Vec::new();
        for i in 0..s_for_two.borrow().entrys.len() as usize {
            for j in
                0..s_for_two.borrow().entrys[0].len()
            {
                let mut var: Vec<f64> = Vec::new();
                for k in 0..s_for_two.borrow().entrys.len() {
                    var.push(s_for_two.borrow().entrys[k][j].text().replace(",",".").parse().unwrap());
                }
                var.shrink_to_fit();
                vars.push(Variable::new(var));
            }
            break;
        }
        let table = DataTable::new(vars);
        create_two_way_window(&application, &table);
    }));
    theory.connect_clicked(clone!(@strong application =>move |_| {
            create_theory_window(&application, fileList[0]);}));
    window.set_child(Some(&main_box));
    window.show();
}
struct matrix {
    entrys: Vec<Vec<gtk::Entry>>,
}

#[cfg(target_os = "windows")]
fn get_path(input: PathBuf) -> String {
    let mut out = String::new();
    let st = input.canonicalize().unwrap().to_str().unwrap().to_string();
    for (i, ch) in st.char_indices() {
        if i > 3 {
            if !(st.chars().nth(i - 1).unwrap() == '\\' && ch == '\\') {
                if ch == '\\' {
                    out.push('/');
                } else {
                    out.push(ch);
                }
            }
        }
    }
    out
}
#[cfg(target_os = "linux")]
fn get_path(input: PathBuf) -> String {
    input
        .canonicalize()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
fn create_theory_window(application: &gtk::Application, file: &str) {
    let window = gtk::Window::builder()
        .application(application)
        .focus_on_click(true)
        .sensitive(true)
        .resizable(true)
        .startup_id("Theory")
        .modal(false)
        .build();

    window.set_title(Some("Theory"));
    window.set_default_size(400, 600);

    let main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    main_box.set_margin_top(15);
    main_box.set_margin_bottom(15);
    main_box.set_margin_start(15);
    main_box.set_margin_end(15);
    main_box.set_valign(gtk::Align::Center);
    let control_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let view = gtk::DrawingArea::new();
    view.set_valign(gtk::Align::Center);
    view.set_halign(gtk::Align::Center);
    view.set_hexpand(true);
    view.set_vexpand(true);
    let data_file = PathBuf::from(format!("./static/{}.pdf", file).as_str());
    let clon = data_file.clone();
    let doc =
        poppler::Document::from_file(format!("file:///{}", get_path(data_file)).as_str(), None)
            .unwrap();
    let mut index: Rc<RefCell<IndexControl>> = Rc::new(RefCell::new(IndexControl {
        cur: 0_i32,
        max: doc.n_pages() - 1,
    }));
    let mut index_draw: Rc<RefCell<IndexControl>> = Rc::clone(&index);
    let mut index_back: Rc<RefCell<IndexControl>> = Rc::clone(&index);
    let mut index_next: Rc<RefCell<IndexControl>> = Rc::clone(&index);
    view.set_draw_func(clone!(@strong window =>
        move |area: &gtk::DrawingArea, ctx: &gtk::cairo::Context, mut w: i32, mut h: i32| {
            let page = doc.page(index_draw.borrow_mut().cur).unwrap();
            ctx.set_source_rgb(0.72, 0.72, 0.72);
            ctx.fill().unwrap();
            let (ww, wh) = page.size();
            w = ww as i32;
            h = wh as i32;
            area.set_size_request(w, h);
            ctx.rectangle(0.0, 0.0, ww, wh);
            ctx.set_source_rgb(1.0, 1.0, 1.0);
            ctx.fill().unwrap();
            page.render(ctx);
            ctx.show_page().unwrap();
            area.queue_draw();
        }
    ));
    let back = gtk::Button::builder()
        .height_request(30)
        .width_request(100)
        .label("back")
        .halign(gtk::Align::Start)
        .build();
    let next = gtk::Button::builder()
        .height_request(30)
        .width_request(100)
        .label("next")
        .halign(gtk::Align::End)
        .build();
    let index_count = Rc::clone(&index);
    let counter = gtk::Label::builder()
        .halign(gtk::Align::Center)
        .label(format!("{}/{}", index_count.borrow().cur, index_count.borrow().max).as_str())
        .build();

    back.connect_clicked(clone!(@strong counter, @strong view => move |_| {
        if index_back.borrow().cur != 0 {
            index_back.borrow_mut().cur -= 1;
            counter.set_text(
                format!(
                    "{}/{}",
                    index_back.borrow().cur,
                    index_back.borrow().max
                )
                .as_str(),
            );
            view.queue_draw();
        }
    }));
    next.connect_clicked(clone!(@strong counter, @strong view =>move |_| {
        if index_next.borrow().cur != index_next.borrow().max {
            index_next.borrow_mut().cur += 1;
            counter.set_text(
                format!(
                    "{}/{}",
                    index_next.borrow().cur,
                    index_next.borrow().max
                )
                .as_str(),
            );
            view.queue_draw();
        }
    }));
    let open = gtk::Button::builder()
        .height_request(30)
        .width_request(100)
        .label("Open")
        .halign(gtk::Align::End)
        .build();

    open.connect_clicked(move |_| {
        execute::shell(get_path(clon.clone()).as_str())
            .spawn()
            .unwrap();
    });
    control_box.append(&back);
    control_box.append(&counter);
    control_box.append(&next);
    control_box.append(&open);
    control_box.set_valign(gtk::Align::Center);
    main_box.append(&control_box);
    let sc_window = gtk::ScrolledWindow::new();
    sc_window.set_valign(gtk::Align::Center);
    sc_window.set_halign(gtk::Align::Center);
    sc_window.set_hscrollbar_policy(gtk::PolicyType::Automatic);
    sc_window.set_vscrollbar_policy(gtk::PolicyType::Automatic);
    sc_window.set_hexpand(true);
    sc_window.set_vexpand(true);
    sc_window.set_size_request(900, 900);
    sc_window.set_child(Some(&view));
    main_box.append(&sc_window);
    window.set_child(Some(&main_box));
    window.show();
}
struct IndexControl {
    cur: i32,
    max: i32,
}
fn create_two_way_window(application: &gtk::Application, table: &DataTable) {
    let result = table.two_way_without_reps();
    let window = gtk::Window::builder()
        .application(application)
        .focus_on_click(true)
        .sensitive(true)
        .resizable(true)
        .startup_id("Two way")
        .modal(false)
        .build();
    window.set_title(Some("One way ANOVA"));
    window.set_default_size(900, 720);
    let main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    main_box.set_margin_top(15);
    main_box.set_margin_bottom(15);
    main_box.set_margin_start(15);
    main_box.set_margin_end(15);
    let control_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let alfa = gtk::Button::builder()
        .label("Alpha")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(100)
        .height_request(20)
        .visible(true)
        .build();
    let alfa_count = gtk::ComboBoxText::builder()
        .height_request(20)
        .width_request(100)
        .build();
    for v in fileList[1..6].iter() {
        alfa_count.append_text(*v);
    }
    alfa.connect_clicked(clone!(@strong application, @strong alfa_count =>move |_| {
            create_theory_window(&application, &alfa_count.active_text().unwrap());}));
    control_box.append(&alfa_count);
    control_box.append(&alfa);
    main_box.append(&control_box);
    let mut flow_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .selection_mode(gtk::SelectionMode::None)
        .min_children_per_line(5)
        .max_children_per_line(5)
        .build();
    flow_box.insert(&gtk::Label::new(Some("Итоги")), -1);
    flow_box.insert(&gtk::Label::new(Some("Счет")), -1);
    flow_box.insert(&gtk::Label::new(Some("Сумма")), -1);
    flow_box.insert(&gtk::Label::new(Some("Среднее")), -1);
    flow_box.insert(&gtk::Label::new(Some("Дисперсия")), -1);
    for var in 0..table.groups.len() {
        flow_box.insert(
            &gtk::Label::new(Some(format!("Столбец {}", var + 1).as_str())),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{}", table.count_of_groups_by_columns[var]).as_str(),
            )),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{:.06}", table.sum_of_groups_by_columns[var]).as_str(),
            )),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{:.06}", table.mean_of_groups_by_columns[var]).as_str(),
            )),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{:.06}", table.dispersion_of_groups_by_columns[var]).as_str(),
            )),
            -1,
        );
    }
    flow_box.insert(&gtk::Label::new(Some("")), -1);
    flow_box.insert(&gtk::Label::new(Some("")), -1);
    flow_box.insert(&gtk::Label::new(Some("")), -1);
    flow_box.insert(&gtk::Label::new(Some("")), -1);
    flow_box.insert(&gtk::Label::new(Some("")), -1);
    for var in 0..table.count_of_groups_by_rows.len() {
        flow_box.insert(
            &gtk::Label::new(Some(format!("Строка {}", var + 1).as_str())),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{}", table.count_of_groups_by_rows[var]).as_str(),
            )),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{:.06}", table.sum_of_groups_by_rows[var]).as_str(),
            )),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{:.06}", table.mean_of_groups_by_rows[var]).as_str(),
            )),
            -1,
        );
        flow_box.insert(
            &gtk::Label::new(Some(
                format!("{:.06}", table.dispersion_of_groups_by_rows[var]).as_str(),
            )),
            -1,
        );
    }
    main_box.append(&flow_box);
    let mut disp_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .selection_mode(gtk::SelectionMode::None)
        .min_children_per_line(6)
        .max_children_per_line(6)
        .build();
    disp_box.insert(&gtk::Label::new(Some("Источник вариации")), -1);
    disp_box.insert(&gtk::Label::new(Some("SS")), -1);
    disp_box.insert(&gtk::Label::new(Some("df")), -1);
    disp_box.insert(&gtk::Label::new(Some("MS")), -1);
    disp_box.insert(&gtk::Label::new(Some("F")), -1);
    disp_box.insert(&gtk::Label::new(Some("p-значение")), -1);
    disp_box.insert(&gtk::Label::new(Some("Строки")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.rows).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.rows).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ms.rows).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.f.rows).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.p.rows).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("Столбцы")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.cols).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.cols).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ms.cols).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.f.cols).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.p.cols).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("Погрешность")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.error).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.error).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ms.error).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("Итоги")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.sum).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.sum).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    main_box.append(&disp_box);
    let scrolled_window = gtk::ScrolledWindow::builder() // Disable horizontal scrolling
        .min_content_width(200)
        .min_content_height(200)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .width_request(900)
        .height_request(600)
        .has_frame(true)
        .visible(true)
        .child(&main_box)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
fn create_one_way_window(application: &gtk::Application, table: &DataTable, by_columns: bool) {
    let result = table.one_way(by_columns);

    let window = gtk::Window::builder()
        .application(application)
        .focus_on_click(true)
        .sensitive(true)
        .resizable(true)
        .startup_id("One way")
        .modal(false)
        .build();
    window.set_title(Some("One way ANOVA"));
    window.set_default_size(900, 720);
    let main_box: gtk::Box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    main_box.set_margin_top(15);
    main_box.set_margin_bottom(15);
    main_box.set_margin_start(15);
    main_box.set_margin_end(15);
    let control_box: gtk::Box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let alfa = gtk::Button::builder()
        .label("Alpha")
        .halign(gtk::Align::End)
        .valign(gtk::Align::Start)
        .width_request(100)
        .height_request(20)
        .visible(true)
        .build();
    let alfa_count = gtk::ComboBoxText::builder()
        .height_request(20)
        .width_request(100)
        .build();
    for v in fileList[1..6].iter() {
        alfa_count.append_text(*v);
    }
    alfa.connect_clicked(clone!(@strong application, @strong alfa_count =>move |_| {
            create_theory_window(&application, &alfa_count.active_text().unwrap());}));
    control_box.append(&alfa_count);
    control_box.append(&alfa);
    main_box.append(&control_box);
    let label = gtk::Label::new(Some("ИТОГИ"));
    main_box.append(&label);
    println!("{:#?}", table.groups);
    if by_columns {
        let mut flow_box = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .selection_mode(gtk::SelectionMode::None)
            .min_children_per_line(5)
            .max_children_per_line(5)
            .build();
        flow_box.insert(&gtk::Label::new(Some("Группы")), -1);
        flow_box.insert(&gtk::Label::new(Some("Счет")), -1);
        flow_box.insert(&gtk::Label::new(Some("Сумма")), -1);
        flow_box.insert(&gtk::Label::new(Some("Среднее")), -1);
        flow_box.insert(&gtk::Label::new(Some("Дисперсия")), -1);
        for var in 0..table.groups.len() {
            flow_box.insert(
                &gtk::Label::new(Some(format!("Столбец {}", var + 1).as_str())),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{}", table.count_of_groups_by_columns[var]).as_str(),
                )),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{:.06}", table.sum_of_groups_by_columns[var]).as_str(),
                )),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{:.06}", table.mean_of_groups_by_columns[var]).as_str(),
                )),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{:.06}", table.dispersion_of_groups_by_columns[var]).as_str(),
                )),
                -1,
            );
        }
        main_box.append(&flow_box);
    } else {
        let mut flow_box = gtk::FlowBox::builder()
            .valign(gtk::Align::Start)
            .selection_mode(gtk::SelectionMode::None)
            .min_children_per_line(5)
            .max_children_per_line(5)
            .build();
        flow_box.insert(&gtk::Label::new(Some("Группы")), -1);
        flow_box.insert(&gtk::Label::new(Some("Счет")), -1);
        flow_box.insert(&gtk::Label::new(Some("Сумма")), -1);
        flow_box.insert(&gtk::Label::new(Some("Среднее")), -1);
        flow_box.insert(&gtk::Label::new(Some("Дисперсия")), -1);
        for var in 0..table.count_of_groups_by_rows.len() {
            flow_box.insert(
                &gtk::Label::new(Some(format!("Строка {}", var + 1).as_str())),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{}", table.count_of_groups_by_rows[var]).as_str(),
                )),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{:.06}", table.sum_of_groups_by_rows[var]).as_str(),
                )),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{:.06}", table.mean_of_groups_by_rows[var]).as_str(),
                )),
                -1,
            );
            flow_box.insert(
                &gtk::Label::new(Some(
                    format!("{:.06}", table.dispersion_of_groups_by_rows[var]).as_str(),
                )),
                -1,
            );
        }
        main_box.append(&flow_box);
    }
    let delim = gtk::Label::new(Some("Дисперсионный анализ"));
    main_box.append(&delim);
    let mut disp_box = gtk::FlowBox::builder()
        .valign(gtk::Align::Start)
        .selection_mode(gtk::SelectionMode::None)
        .min_children_per_line(6)
        .max_children_per_line(6)
        .build();
    disp_box.insert(&gtk::Label::new(Some("Источник вариации")), -1);
    disp_box.insert(&gtk::Label::new(Some("SS")), -1);
    disp_box.insert(&gtk::Label::new(Some("df")), -1);
    disp_box.insert(&gtk::Label::new(Some("MS")), -1);
    disp_box.insert(&gtk::Label::new(Some("F")), -1);
    disp_box.insert(&gtk::Label::new(Some("p-значение")), -1);
    disp_box.insert(&gtk::Label::new(Some("Между группами")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.between).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.between).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ms.between).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.f).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.p).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("Внутри групп")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.inside).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.inside).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ms.inside).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("")), -1);

    disp_box.insert(&gtk::Label::new(Some("Итоги")), -1);
    disp_box.insert(
        &gtk::Label::new(Some(format!("{:.06}", result.ss.sum).as_str())),
        -1,
    );
    disp_box.insert(
        &gtk::Label::new(Some(format!("{}", result.df.sum).as_str())),
        -1,
    );
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    disp_box.insert(&gtk::Label::new(Some("")), -1);
    main_box.append(&disp_box);
    let scrolled_window = gtk::ScrolledWindow::builder() // Disable horizontal scrolling
        .min_content_width(200)
        .min_content_height(200)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .width_request(900)
        .height_request(600)
        .has_frame(true)
        .visible(true)
        .child(&main_box)
        .build();
    window.set_child(Some(&scrolled_window));
    window.show();
}
