use std::collections::BTreeMap;

use std::fs;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;


use eframe::NativeOptions;

use egui::FontId;
use egui::RichText;
use egui::Ui;
use std::process::Command;
use std::str::FromStr;
#[derive(Default)]
struct App {
    prev_total_cpu_time: f32,
    prev_idle_cpu_time: f32,
    is_list_mode: bool,
    processes_data: BTreeMap<u32, ProcInfo>,
    total_cpu_usage: Arc<Mutex<f32>>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, process_data: BTreeMap<u32, ProcInfo>, mut total_cpu_usage: Arc<Mutex<f32>>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
         Self {
            prev_total_cpu_time: 0.,
            prev_idle_cpu_time: 0.,
            is_list_mode: true,
            processes_data: process_data,
            total_cpu_usage: total_cpu_usage,
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
        let total_rows = self.processes_data.len();

        egui::ScrollArea::vertical().auto_shrink(false).show_rows(
            ui,
            row_height,
            total_rows,
            |ui: &mut Ui, total_rows: std::ops::Range<usize>| {
                let process_vec: Vec<_> = self.processes_data.values().collect();
                for i in total_rows {
                    ui.horizontal(|ui| {
                        ui.columns(7, |columns| {
                            columns[0].label(RichText::new(format!("{}", process_vec[i].name)));
                            columns[1].label(RichText::new(format!("{}", process_vec[i].user)));
                            columns[2].label(RichText::new(format!("{}", process_vec[i].pid)));
                            columns[3].label(RichText::new(format!("{}", process_vec[i].status)));
                            columns[4].label(RichText::new(format!("{:.2}%", process_vec[i].cpu)));
                            columns[5]
                                .label(RichText::new(format!("{} kb", process_vec[i].memory_used)));
                            columns[6].label(RichText::new(format!("{}", process_vec[i].path)));
                        });
                    });
                }
            },
        );
    }
    fn show_rows_as_tree(&self, ui: &mut Ui) {
        let text_style = egui::TextStyle::Body;
        let row_height = ui.text_style_height(&text_style);
        let total_rows = 0; //self.processes_data.len();
        
        egui::ScrollArea::vertical().auto_shrink(false).show_rows(
            ui,
            row_height,
            total_rows,
            |ui: &mut Ui, total_rows: std::ops::Range<usize>| {
                for process in self.processes_data.values() {
                    if process.is_children {
                        continue;
                    }
                    self.create_collapse_area(ui, process)
                }
            },
        );
    }
    fn create_collapse_area(&self, ui: &mut Ui, process: &ProcInfo) {
        if process.children_processes.is_empty() {
            ui.label(RichText::new(format!{"{} | {} | {} | {} | {:.2}% | {} kb | {}",process.name,process.user,process.pid,process.status,process.cpu,process.memory_used,process.path}));
        } else {
            ui.collapsing(RichText::new(format!{"{} | {} | {} | {} | {:.2}% | {} kb | {}",process.name,process.user,process.pid,process.status,process.cpu,process.memory_used,process.path}), |ui| {
                
                for child in &process.children_processes
                {
                    if let Some(child_process) = self.processes_data.get(child)
                    {
                        self.create_collapse_area(ui, child_process)
                    }
                }
            });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.horizontal(|ui| {
                if let Ok(locked_value) = self.total_cpu_usage.lock() {
                    ui.label(format!("Total cpu usage: {:.2}%",locked_value));
                }
                else
                {
                    ui.label(format!("Unable to get cpu usage!"));
                }
            });
            let mut button_message = String::from("List view");
            if self.is_list_mode {
                button_message = String::from("Tree view");
            }

            if ui.button(button_message).clicked() {
                self.is_list_mode = !self.is_list_mode;
            }

            self.create_header_row(ui);
            if self.is_list_mode {
                self.show_rows_as_list(ui);
            } else {
                self.show_rows_as_tree(ui)
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
    is_children: bool,
}
#[derive(Clone)]
struct ProcInfo {
    name: String,
    user: String,
    pid: u32,
    status: String,
    cpu: f32,
    memory_used: u32,
    path: String,
    children_processes: Vec<u32>,
    is_children: bool,
}
//################################################################
fn read_process_info(pid: u32) -> io::Result<Info> {
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(status_path)?;

    let mut name = String::new();
    let mut status = String::new();
    let mut memory_used: u32 = 0;
    let mut is_children: bool = false;

    for line in status_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            match parts[0] {
                "Name:" => name = parts[1].to_string(),
                "State:" => status = parts[1].to_string(),
                "VmRSS:" => memory_used = parts[1].parse().unwrap(),
                "PPid:" => {
                    if parts[1].parse::<u32>().unwrap() != 0 {
                        is_children = true;
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
        is_children,
    })
}
//################################################################
fn read_cpu_usage(prev_total_time: &mut f32, prev_idle_time: &mut f32) -> io::Result<f32> {
    // read the first line of   /proc/stat
    let status_content = fs::read_to_string("/proc/stat")?;
    let cpu_line_info = status_content.lines().next().unwrap();

    let mut idle_time: f32 = 0_f32;
    let mut total_time: f32 = 0_f32;

    for data in cpu_line_info.split_whitespace().enumerate() {
        // discard the first word of that first line   (it's always cpu)
        if data.0 == 0 {
            continue;
        }
        let data_as_number: f32 = FromStr::from_str(data.1).unwrap();
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
//https://stackoverflow.com/questions/16726779/how-do-i-get-the-total-cpu-usage-of-an-application-from-proc-pid-stat
fn get_process_cpu_usage(pid: u32) -> io::Result<f32> {
    let path = format!("/proc/{}/stat", pid);
    let stat_file = fs::read_to_string(path)?;
    let path_uptime = fs::read_to_string("/proc/uptime")?;

    let fields: Vec<&str> = stat_file.split_whitespace().collect();
    let field_uptime: Vec<&str> = path_uptime.split_whitespace().collect();

    let uptime: f32 = field_uptime[0].parse().unwrap();
    let utime: u64 = fields[12].parse().unwrap();
    let stime: u64 = fields[13].parse().unwrap();
    //if we want to include children processes, we need to get fields 14 and 15 too.

    let start_time: u64 = fields[21].parse().unwrap();
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
        let result = String::from(
            all_data
                .lines()
                .last()
                .unwrap()
                .split_whitespace()
                .nth(2)
                .unwrap(),
        );
        Ok(result)
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
        memory_used: 0,
        path: String::from(""),
        children_processes: Vec::new(),
        is_children: false,
    };

    if let Ok(info) = read_process_info(pid) {
        proc_info.status = info.status;
        proc_info.pid = info.pid;
        proc_info.memory_used = info.memory_used;
        proc_info.name = info.name;
        proc_info.is_children = info.is_children;
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
    let mut processes_data: BTreeMap<u32, ProcInfo> = BTreeMap::new();

    let proc_path = "/proc";

    if let Ok(entries) = fs::read_dir(proc_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
                let proc_info = get_process_data(pid);
                processes_data.insert(pid, proc_info);
            }
        }
    }
    let total_cpu_usage_mutex = Arc::new(Mutex::new(0.));
    let total_cpu_usage_mutex_clone = total_cpu_usage_mutex.clone();

  
    thread::spawn(move || {
        let mut previous_cpu_usage = 0_f32;
        let mut previous_idle_time = 0_f32;
        let thread_total_cpu_usage = Arc::clone(&total_cpu_usage_mutex);
        loop
        {
            if let Ok(total_cpu) = read_cpu_usage(&mut previous_cpu_usage, &mut previous_idle_time)
            {
                if let Ok(mut total_cpu_usage) = thread_total_cpu_usage.lock()
                {
                    *total_cpu_usage = total_cpu;
                }
            }
            thread::sleep(Duration::new(2, 0));
        }
    });

    let native_options = NativeOptions::default();
    match eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(move |cc| Box::new(App::new(cc, processes_data, Arc::clone(&total_cpu_usage_mutex_clone)))),
    ) {
        Ok(()) => println!("Running!"),
        Err(error) => println!("Error: {}", error),
    }
}
