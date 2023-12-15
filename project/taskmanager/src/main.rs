use std::fs;
use std::io;

use eframe::NativeOptions;
use egui::Color32;
use egui::FontId;
use egui::RichText;
use egui::Ui;
use std::process::Command;
use std::str::FromStr;
#[derive(Default)]
struct App {
    prev_total_cpu_time: f32,
    prev_idle_cpu_time: f32,
    isFirstTime: bool,
    processes_data: Vec<ProcInfo>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, process_data: Vec<ProcInfo>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            prev_total_cpu_time: 0.,
            prev_idle_cpu_time: 0.,
            isFirstTime: true,
            processes_data: process_data,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "Total cpu usage: {:.2}%",
                    read_cpu_usage(&mut self.prev_total_cpu_time, &mut self.prev_idle_cpu_time)
                        .unwrap()
                ));
            });

            ui.horizontal(|ui| {
                ui.columns(7, |columns| {
                    columns[0].label(
                        RichText::new("Name")
                            .font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                    columns[1].label(
                        RichText::new("User")
                            .font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                    columns[2].label(
                        RichText::new("PID").font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                    columns[3].label(
                        RichText::new("Status")
                            .font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                    columns[4].label(
                        RichText::new("CPU%")
                            .font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                    columns[5].label(
                        RichText::new("Mem").font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                    columns[6].label(
                        RichText::new("Path")
                            .font(FontId::new(20., egui::FontFamily::Proportional)),
                    );
                });
            });

            let text_style = egui::TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let total_rows = self.processes_data.len();

            egui::ScrollArea::vertical().auto_shrink(false).show_rows(
                ui,
                row_height,
                total_rows,
                |ui: &mut Ui, total_rows: std::ops::Range<usize>| {
                    for i in total_rows {

                        ui.horizontal(|ui| {
                            ui.columns(7, |columns| {
                                columns[0].label(
                                    RichText::new(format!("{}", &self.processes_data[i].name)),
                                );
                                columns[1].label(
                                    RichText::new(format!("{}", &self.processes_data[i].user)),
                                );
                                columns[2].label(
                                    RichText::new(format!("{}", &self.processes_data[i].pid)),
                                );
                                columns[3].label(
                                    RichText::new(format!("{}", &self.processes_data[i].status)),
                                );
                                columns[4].label(
                                    RichText::new(format!("{:.2}%", &self.processes_data[i].cpu)),
                                );
                                columns[5].label(
                                    RichText::new(format!("{} kb", &self.processes_data[i].memory_used)),
                                );
                                columns[6].label(
                                    RichText::new(format!("{}", &self.processes_data[i].path)),
                                );
                            });
                        });
                    }
                },
            );
        });

        // sleep(Duration::new(5, 0));
    }
}

struct Info {
    pid: u32,
    name: String,
    status: String,
    memory_used: u32,
}
struct ProcInfo {
    name: String,
    user: String,
    pid: u32,
    status: String,
    cpu: f32,
    memory_used: u32,
    path: String,
}
//################################################################
fn read_process_info(pid: u32) -> io::Result<Info> {
    let status_path = format!("/proc/{}/status", pid);
    let status_content = fs::read_to_string(status_path)?;

    let mut name = String::new();
    let mut status = String::new();
    let mut memory_used: u32 = 0;

    for line in status_content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            match parts[0] {
                "Name:" => name = parts[1].to_string(),
                "State:" => status = parts[1].to_string(),
                "VmRSS:" => memory_used = parts[1].parse().unwrap(),
                _ => (),
            }
        }
    }

    Ok(Info {
        pid,
        name,
        status,
        memory_used,
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
fn get_children_processes(pid: u32) -> io::Result<String> {
    let path = format!("/proc/{}/task/{}/children", pid, pid);
    let children_processes = fs::read_to_string(path)?;

    if children_processes.is_empty() {
        return Ok(format!("{}", pid));
    }
    let mut process_tree = format!("{}(", pid);
    for child in children_processes.split_whitespace() {
        match get_children_processes(child.parse().unwrap()) {
            Ok(child_tree) => process_tree.push_str(&child_tree),
            Err(error) => {
                println!("Error at get_child_process({}): {}", pid, error);
            }
        }
        process_tree.push(',');
    }
    process_tree.pop().unwrap();
    process_tree.push(')');
    Ok(process_tree)
}
//################################################################
fn main() {
    let mut processes_data: Vec<ProcInfo> = Vec::new();

    let proc_path = "/proc";

    if let Ok(entries) = fs::read_dir(proc_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
                let mut proc_info: ProcInfo = ProcInfo {
                    name: String::from(""),
                    user: String::from(""),
                    pid: 0,
                    status: String::from(""),
                    cpu: 0.,
                    memory_used: 0,
                    path: String::from(""),
                };

                if let Ok(info) = read_process_info(pid) {
                    proc_info.status = info.status;
                    proc_info.pid = info.pid;
                    proc_info.memory_used = info.memory_used;
                    proc_info.name = info.name;
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
                processes_data.push(proc_info);
            }
        }
    }
    let native_options = NativeOptions::default();
    match eframe::run_native(
        "Task Manager",
        native_options,
        Box::new(|cc| Box::new(App::new(cc, processes_data))),
    ) {
        Ok(()) => println!("Running!"),
        Err(error) => println!("Error: {}", error),
    }
}
