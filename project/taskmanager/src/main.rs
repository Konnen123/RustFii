use eframe::NativeOptions;
use egui::Color32;
use egui::FontId;
use egui::RichText;
use egui::Ui;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::process::Command;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
#[derive(Default)]
struct App {
    is_list_mode: bool,
    is_process_mode: bool,
    show_all_procesess: bool,
    process_data_mutex: Arc<Mutex<BTreeMap<u32, ProcInfo>>>,
    total_cpu_usage: Arc<Mutex<f32>>,
    memory_info: Arc<Mutex<(f32, f32)>>,
}

impl App {
    fn new(
        cc: &eframe::CreationContext<'_>,
        process_data_mutex: Arc<Mutex<BTreeMap<u32, ProcInfo>>>,
        total_cpu_usage: Arc<Mutex<f32>>,
        memory_info: Arc<Mutex<(f32, f32)>>,
    ) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        if let Some(cpu_usage) = cc.integration_info.cpu_usage {
            println!("Cpu usage: {}", cpu_usage);
        }
        Self {
            is_list_mode: true,
            is_process_mode: true,
            show_all_procesess: false,
            process_data_mutex,
            total_cpu_usage,
            memory_info,
        }
    }

    fn create_header_row(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.columns(7, |columns| {
                columns[0].label(
                    RichText::new("Name").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
                columns[1].label(
                    RichText::new("User").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
                columns[2].label(
                    RichText::new("PID").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
                columns[3].label(
                    RichText::new("Status").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
                columns[4].label(
                    RichText::new("CPU%").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
                columns[5].label(
                    RichText::new("Mem").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
                columns[6].label(
                    RichText::new("Path").font(FontId::new(20., egui::FontFamily::Proportional)),
                );
            });
        });
    }
    fn show_rows_as_list(&self, ui: &mut Ui) {
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let mut total_rows: usize = 0;
        match self.process_data_mutex.lock() {
            Ok(process_map) => {
                if !self.show_all_procesess
                {
                    let x: Vec<_>= process_map.values().filter(|process| process.user != "root").collect();
                    total_rows = x.len();
                }
                else {
                    total_rows = process_map.len();
                }
            }
            Err(error) => {
                println!("Error at getting process_data length: {error}. The total_rows will be {total_rows}, so we exit function!");
                return;
            }
        }

        egui::ScrollArea::vertical().auto_shrink(false).show_rows(
            ui,
            row_height,
            total_rows,
            |ui: &mut Ui, total_rows: std::ops::Range<usize>| {
                if let Ok(process_map) = self.process_data_mutex.lock() {

                let process_vec: Vec<_> = if !self.show_all_procesess
                {
                    process_map.values().filter(|process| process.user != "root").collect()

                }
                else {
                     process_map.values().collect()
                };
                    for i in total_rows {
                        ui.horizontal(|ui| {
                            ui.columns(7, |columns| {
                                if let Some(process) = process_vec.get(i)
                                {

                                    columns[0].label(RichText::new(process.name.to_string()));
                                    columns[1].label(RichText::new(process.user.to_string()));
                                    columns[2].label(RichText::new(format!("{}",process.pid)));
                                    columns[3].label(RichText::new(process.status.to_string()));
                                    columns[4]
                                    .label(RichText::new(format!("{:.2}%", process.cpu)));
                                columns[5].label(RichText::new(format!(
                                    "{:.2} Mb",
                                    process.memory_used
                                )));
                                columns[6].label(RichText::new(process.path.to_string()));
                            }
                            });
                        });
                    }
                } else {
                    println!("Error at locking the mutex in the function show_rows_as_list!");
                }
            },
        );
    }
    fn show_rows_as_tree(&mut self, ui: &mut Ui) {
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let total_rows = 0;

        egui::ScrollArea::vertical().auto_shrink(false).show_rows(
            ui,
            row_height,
            total_rows,
            |ui: &mut Ui, total_rows: std::ops::Range<usize>| {
                total_rows.is_empty();
                if let Ok(process_map) = self.process_data_mutex.lock() {
                    
                    let process_vec: Vec<_> = if !self.show_all_procesess
                    {
                        process_map.values().filter(|process| process.user != "root").collect()
                    }
                    else {
                        process_map.values().collect()
                    };

                    for process in &process_vec {
                        if process_vec.clone().into_iter().any(|proc| process.parent_pid == proc.pid)
                        {
                            continue;
                        }
                        let values: std::collections::btree_map::Values<'_, u32, ProcInfo> =
                        process_map.values();
                        self.create_collapse_area(ui, process, values);
                    }
                } else {
                    println!("Error at getting process_map from tree view!");
                }
            },
        );
    }
    fn create_collapse_area(
        &self,
        ui: &mut Ui,
        process: &ProcInfo,
        mut values: std::collections::btree_map::Values<'_, u32, ProcInfo>,

    ) {
        if process.children_processes.is_empty() {
            ui.label(RichText::new(format!{"{} | {} | {} | {} | {:.2}% | {:.2} Mb | {}",process.name,process.user,process.pid,process.status,process.cpu,process.memory_used,process.path}));
        } else {

            ui.collapsing(RichText::new(format!{"{} | {} | {} | {} | {:.2}% | {:.2} Mb | {}",process.name,process.user,process.pid,process.status,process.cpu,process.memory_used,process.path}), |ui| {
                    for child in &process.children_processes
                    { 
                        if let Some(child_process) = values.find(|proc_info| { proc_info.pid == *child })
                        {
                            self.create_collapse_area(ui, child_process, values.clone());
                        }
                    }
                
            });
        }
    }
    fn show_performance(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            if let Ok(locked_value) = self.total_cpu_usage.lock() {
                ui.label(format!("Total cpu usage: {:.2}%", locked_value));
                let progress_bar = egui::ProgressBar::new(*locked_value / 100.0).animate(false);
                ui.add(progress_bar);
            } else {
                ui.label("Unable to get cpu usage!");
            }
        });
        ui.horizontal(|ui| {
            if let Ok(memory_used) = self.memory_info.lock() {
                ui.label(format!("Total memory used {:.2} GB", memory_used.1));
                let progress_bar = egui::ProgressBar::new(memory_used.1 / memory_used.0)
                    .fill(Color32::RED)
                    .animate(false);
                ui.add(progress_bar);
            } else {
                ui.label("Unable to get memory usage!");
            }
        });
    }
    fn show_processes(&mut self, ui: &mut Ui) {
        let mut button_message = String::from("List view");
        if self.is_list_mode {
            button_message = String::from("Tree view");
        }
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.y = 16.0;
        });
        ui.horizontal(|ui| {
            if ui.button(button_message).clicked() {
                self.is_list_mode = !self.is_list_mode;
            }
            ui.checkbox(&mut self.show_all_procesess, "Show all processes");
        });

        self.create_header_row(ui);
        if self.is_list_mode {
            self.show_rows_as_list(ui);
        } else {
            self.show_rows_as_tree(ui)
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.is_web();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Processes").clicked() {
                    self.is_process_mode = true;
                }
                if ui.button("Performance").clicked() {
                    self.is_process_mode = false;
                }
            });

            if self.is_process_mode {
                self.show_processes(ui);
            } else {
                self.show_performance(ui);
            }
            ctx.request_repaint();
        });
    }
}

struct Info {
    pid: u32,
    name: String,
    status: String,
    memory_used: u32,
    parent_pid: u32,
}
#[derive(Clone)]
struct ProcInfo {
    name: String,
    user: String,
    pid: u32,
    status: String,
    cpu: f32,
    memory_used: f32,
    path: String,
    children_processes: Vec<u32>,
    parent_pid: u32,
}
//################################################################
fn read_process_info(pid: u32) -> io::Result<Info> {
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(status_path)?;

    let mut name = String::new();
    let mut status = String::new();
    let mut memory_used: u32 = 0;
    let mut parent_pid: u32 = 0;

    for line in status_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            match parts[0] {
                "Name:" => name = parts[1].to_string(),
                "State:" => status = parts[1].to_string(),
                "VmRSS:" => {
                    if let Ok(value) = parts[1].parse::<u32>() {
                        memory_used = value;
                    }
                }
                "PPid:" => {
                    if let Ok(value) = parts[1].parse::<u32>() {
                        parent_pid = value;
                    }
                }
                _ => (),
            }
        }
    }

    Ok(Info {
        pid,
        name,
        status,
        memory_used,
        parent_pid,
    })
}
//################################################################
fn read_cpu_usage(prev_total_time: &mut f32, prev_idle_time: &mut f32) -> io::Result<f32> {
    // read the first line of   /proc/stat
    let status_content = fs::read_to_string("/proc/stat")?;
    let cpu_line_info;
    if let Some(info) = status_content.lines().next() {
        cpu_line_info = info;
    } else {
        println!("Error at getting cpu_line_info!");
        return Ok(0.);
    }

    let mut idle_time: f32 = 0_f32;
    let mut total_time: f32 = 0_f32;

    for data in cpu_line_info.split_whitespace().enumerate() {
        // discard the first word of that first line   (it's always cpu)
        if data.0 == 0 {
            continue;
        }
        let mut data_as_number: f32 =0.;
        if let Ok(value) = FromStr::from_str(data.1) {
            data_as_number = value;
        }
        //idle is found at coloumn 5
        if data.0 == 4 {
            idle_time = data_as_number;
        }
        // sum all of the times found on that first line to get the total time
        total_time += data_as_number;
    }

    // multiply by   100   to get a percentage
    let cpu_usage_percentage =
        (1_f32 - (idle_time - *prev_idle_time) / (total_time - *prev_total_time)) * 100_f32;

    *prev_idle_time = idle_time;
    *prev_total_time = total_time;
    Ok(cpu_usage_percentage)
}
//################################################################
fn read_memory_usage() -> io::Result<(f32, f32)> {
    // read the first line of   /proc/stat
    let status_content = fs::read_to_string("/proc/meminfo")?;

    let mut memory_lines = status_content.lines();
    let mut total_memory_kb: f32 = 0.;
    let mut available_memory_kb: f32 = 0.;
    if let Some(total_memory) = memory_lines.next() {
        let string_value_option = total_memory.split_whitespace().nth(1);
        if let Some(string_value) = string_value_option {
            if let Ok(parsed_memory) = string_value.parse::<f32>() {
                total_memory_kb = parsed_memory;
            }
        }
    }

    if let Some(available_memory) = memory_lines.nth(1) {
        let string_value_option = available_memory.split_whitespace().nth(1);
        if let Some(string_value) = string_value_option {
            if let Ok(parsed_memory) = string_value.parse::<f32>() {
                available_memory_kb = parsed_memory;
            }
        }
    }

    Ok((total_memory_kb, total_memory_kb - available_memory_kb))
}
//################################################################
//https://stackoverflow.com/questions/16726779/how-do-i-get-the-total-cpu-usage-of-an-application-from-proc-pid-stat
fn get_process_cpu_usage(pid: u32) -> io::Result<f32> {
    let path = format!("/proc/{}/stat", pid);
    let stat_file = fs::read_to_string(path)?;
    let path_uptime = fs::read_to_string("/proc/uptime")?;

    let fields: Vec<&str> = stat_file.split_whitespace().collect();
    let field_uptime: Vec<&str> = path_uptime.split_whitespace().collect();

    let mut uptime: f32 = 0.;
    let mut utime: u32 = 0;
    let mut stime: u32 = 0;
    let mut start_time: u32 = 0;
    if let Ok(value) = field_uptime[0].parse::<f32>() {
        uptime = value;
    }
    if let Ok(value) = fields[12].parse::<u32>() {
        utime = value;
    }
    if let Ok(value) = fields[13].parse::<u32>() {
        stime = value;
    }
    if let Ok(value) = fields[21].parse::<u32>() {
        start_time = value;
    }
    //if we want to include children processes, we need to get fields 14 and 15 too.

    let total_time = utime + stime;

    let hertz = procfs::ticks_per_second() as f32;
    let seconds = uptime - (start_time as f32 / hertz as f32);

    Ok(100_f32 * ((total_time as f32 / hertz) / seconds))
}
//################################################################
fn get_process_file_path(pid: u32) -> io::Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo readlink /proc/{}/exe", pid))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    } else {
        Ok(String::from("Not found!"))
    }
}
//################################################################
fn get_process_user_name(pid: u32) -> io::Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo ls -l /proc/{}", pid))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let all_data = String::from_utf8_lossy(&output.stdout).to_string();
        if let Some(last_line) = all_data.lines().last() {
            if let Some(user_name) = last_line.split_whitespace().nth(2) {
                let result = String::from(user_name);
                Ok(result)
            }
            else
            {
                Ok(String::from("N/A"))
            }
        }
        else
        {
            Ok(String::from("N/A"))
        }
    } else {
        Ok(String::from("root"))
    }
}
//################################################################
fn get_children_processes(proc_info: &mut ProcInfo) -> io::Result<()> {
    let path = format!("/proc/{}/task/{}/children", proc_info.pid, proc_info.pid);
    let children_processes = fs::read_to_string(path)?;

    if children_processes.is_empty() {
        return Ok(());
    }

    for child in children_processes.split_whitespace() {
        match child.parse::<u32>() {
            Ok(child_pid) => proc_info.children_processes.push(child_pid),
            Err(_) => println!("Error at getting child pid: {}!", child),
        }
    }

    Ok(())
}
//################################################################
fn get_process_data(pid: u32) -> ProcInfo {
    let mut proc_info: ProcInfo = ProcInfo {
        name: String::from(""),
        user: String::from(""),
        pid: 0,
        status: String::from(""),
        cpu: 0.,
        memory_used: 0.,
        path: String::from(""),
        children_processes: Vec::new(),
        parent_pid: 0,
    };

    if let Ok(info) = read_process_info(pid) {
        proc_info.status = info.status;
        proc_info.pid = info.pid;
        proc_info.memory_used = info.memory_used as f32 / 1024.0;
        proc_info.name = info.name;
        proc_info.parent_pid = info.parent_pid;
    }
    if let Ok(process_cpu_usage) = get_process_cpu_usage(pid) {
        proc_info.cpu = process_cpu_usage;
    }
    if let Ok(user_name) = get_process_user_name(pid) {
        proc_info.user = user_name;
    }
    if let Ok(file_path) = get_process_file_path(pid) {
        proc_info.path = file_path;
    }
    if let Err(error) = get_children_processes(&mut proc_info) {
        println!("Error at get_children_processes: {}", error);
    }

    proc_info
}
//################################################################

fn main() {
    let processes_data: BTreeMap<u32, ProcInfo> = BTreeMap::new();
    let processes_data_mutex = Arc::new(Mutex::new(processes_data));
    let processes_data_mutex_clone = processes_data_mutex.clone();

    thread::spawn(move || {
        let proc_path = "/proc";

        loop {
            let mut next_process_map: BTreeMap<u32, ProcInfo> = BTreeMap::new();

            if let Ok(entries) = fs::read_dir(proc_path) {
                for entry in entries.filter_map(|e| e.ok()) {
                    if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
                        let proc_info = get_process_data(pid);
                        next_process_map.insert(pid, proc_info);
                    }
                }
            }
            if let Ok(mut current_process_map) = processes_data_mutex.lock() {
                current_process_map.clear();
                current_process_map.append(&mut next_process_map);
            } else {
                println!("Error at updating process_map!");
            }
            thread::sleep(Duration::new(10, 0));
        }
    });
    let total_cpu_usage_mutex = Arc::new(Mutex::new(0.));
    let total_cpu_usage_mutex_clone = total_cpu_usage_mutex.clone();

    thread::spawn(move || {
        let mut previous_cpu_usage = 0_f32;
        let mut previous_idle_time = 0_f32;
        loop {
            if let Ok(total_cpu) = read_cpu_usage(&mut previous_cpu_usage, &mut previous_idle_time)
            {
                if let Ok(mut total_cpu_usage) = total_cpu_usage_mutex.lock() {
                    *total_cpu_usage = total_cpu;
                }
            }
            thread::sleep(Duration::new(2, 0));
        }
    });

    let total_memory_used_mutex = Arc::new(Mutex::new((0., 0.)));
    let total_memory_used_mutex_clone = total_memory_used_mutex.clone();

    thread::spawn(move || loop {
        if let Ok((total_memory, used_memory)) = read_memory_usage() {
            if let Ok(mut total_memory_used) = total_memory_used_mutex.lock() {
                total_memory_used.0 = total_memory / 1_048_576.0;
                total_memory_used.1 = used_memory / 1_048_576.0;
            }
        }

        thread::sleep(Duration::new(2, 0));
    });

    let native_options = NativeOptions::default();
    match eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(move |cc| {
            Box::new(App::new(
                cc,
                processes_data_mutex_clone,
                total_cpu_usage_mutex_clone,
                total_memory_used_mutex_clone,
            ))
        }),
    ) {
        Ok(()) => println!("Running!"),
        Err(error) => println!("Error: {}", error),
    }
}
