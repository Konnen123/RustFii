use std::fs;
use std::io;

use std::process::Command;
use std::str::FromStr;

struct ProcessInfo {
    pid: u32,
    name: String,
    status: String,
    memory_used: u32,
}
//################################################################
fn read_process_info(pid: u32) -> io::Result<ProcessInfo> {
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

    Ok(ProcessInfo {
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
        if data.0 == 4
        {
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
    let proc_path = "/proc";

    let mut total_memory = 0_u32;

    if let Ok(entries) = fs::read_dir(proc_path) {
        for entry in entries.filter_map(|e| e.ok()) {
            if let Ok(pid) = entry.file_name().to_string_lossy().parse::<u32>() {
                match get_children_processes(pid) {
                    Ok(tree) => println!("{}", tree),
                    Err(error) => println!("Error at get_children_process: {}", error),
                }
                if let Ok(process_info) = read_process_info(pid) {
                    print!(
                        "PID: {} | Name: {} | Status: {} | Memory used: {} kb | ",
                        process_info.pid,
                        process_info.name,
                        process_info.status,
                        process_info.memory_used
                    );
                    total_memory += process_info.memory_used;
                }

                if let Ok(process_cpu_usage) = get_process_cpu_usage(pid) {
                    print!("CPU usage: {:.2}% | ", process_cpu_usage);
                }
                if let Ok(user_name) = get_process_user_name(pid) {
                    print!("User name: {} | ", user_name);
                }
                if let Ok(file_path) = get_process_file_path(pid) {
                    println!("File path: {}", file_path);
                }
                println!("----------------------");
            }
        }
    }
    println!("Total Memory: {} kb", total_memory);
}
