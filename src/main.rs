use regex::Regex;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

#[derive(Debug)]
struct SystemInfo {
    os: Option<String>,
    kernel: Option<String>,

    cpu: Option<String>,

    gpu: Option<String>,
}

impl SystemInfo {
    fn new() -> Self {
        SystemInfo {
            os: None,
            kernel: None,
            cpu: None,
            gpu: None,
        }
    }
}

/// Gets the second item in the line after the ':' and trims it accordingly
fn store_proc_cpuinfo(pointer: &mut String, line: &String) {
    // split the string on the ':', get the second item
    let slices: Vec<&str> = line.split(":").collect();

    // get the string after the : if it exists
    // check that the vector is at least 2 long
    if slices.len() >= 2 {
        *pointer = String::from(slices[1].trim());
    }
}

/// gather the cpu info from /proc/cpuinfo
/// and from /sys/devices/system/cpu/cpu0/cpufreq/bios_limit
fn get_cpu_info(cpu: &mut Option<String>) {
    let proc_file = File::open("/proc/cpuinfo").unwrap();
    let reader = BufReader::new(proc_file);

    let mut model_name: String = Default::default();
    let mut cpu_cores: String = Default::default();

    for line in reader.lines() {
        let line = line.unwrap();

        if model_name == String::default() && line.contains("model name") {
            store_proc_cpuinfo(&mut model_name, &line);
        } else if cpu_cores == String::default() && line.contains("cpu cores") {
            store_proc_cpuinfo(&mut cpu_cores, &line);
        }

        // break out so we dont waste time looking
        if cpu_cores != String::default() && model_name != String::default() {
            break;
        }
    }

    let cpu_freq_file_as_string =
        fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/bios_limit").unwrap();

    // get the string, trim and parse it into a u32
    let cpu_freq_in_khz: u32 = cpu_freq_file_as_string.trim().parse::<u32>().unwrap();
    // change the hz into Ghz
    let cpu_freq_in_ghz: f32 = cpu_freq_in_khz as f32 / f32::powf(10.0, 6.0);

    // build the final string for the CPU information
    *cpu = Some(format!(
        "{} ({}) @ {:.3}GHz",
        model_name, cpu_cores, cpu_freq_in_ghz,
    ));
}

/// gets the GPU info using 'lspci' and formats it, places data string into 'gpu'
fn get_gpu_info(gpu: &mut Option<String>) {
    let command_output = send_bash_command("lspci");

    for line in command_output.lines() {
        if line.contains("VGA compatible controller") {
            let between_brackets_regex = Regex::new(r"\[([^\]]+)\]").unwrap();

            // if there is a match in the line, get the string out of the Some(Match<>)
            if let Some(captures) = between_brackets_regex.captures(line) {
                *gpu = Some(captures.get(1).map_or("", |m| m.as_str()).to_string());
            }
        }
    }
}

/// will execute the bash command passed in,
/// if failed will panic and print error message
fn send_bash_command(command: &str) -> String {
    let bash_command_process = Command::new(command).output();

    let result_string = match bash_command_process {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),

        Err(e) => {
            panic!("Couldnt execute '{}': {}", command, e);
        }
    };

    result_string
}

/// Copy of bash_command_process, but takes params for the command
/// will execute the bash command passed in with the parameters,
/// if failed will panic and print error message
fn send_bash_command_with_params(command: &str, parameters: &[&str]) -> String {
    let bash_command_process = Command::new(command).args(parameters).output();

    let result_string = match bash_command_process {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),

        Err(e) => {
            panic!("Couldnt execute '{}': {}", command, e);
        }
    };

    result_string
}

/// get kernel info using 'uname -r'
fn get_kernel_info(kernel: &mut Option<String>) {
    *kernel = Some(send_bash_command_with_params("uname", &["-r"]));
}

/// gets the os info
/// will panic on fail
fn get_os(os: &mut Option<String>) {
    let mut pretty_name_os: String = send_bash_command_with_params("cat", &["/etc/os-release"]);

    pretty_name_os = pretty_name_os
        .lines()
        .find(|line| line.contains("PRETTY_NAME"))
        .map(|line| line.to_string())
        .unwrap();

    let between_quotes_regex = Regex::new(r#""([^"]*)""#).unwrap();

    // panic if cant get between quotes
    let captures = between_quotes_regex.captures(&pretty_name_os).unwrap();

    pretty_name_os = captures.get(1).map_or("", |m| m.as_str()).to_string();

    let architechture = send_bash_command_with_params("uname", &["-m"]);

    *os = Some(pretty_name_os + " " + &architechture);
}

fn main() {
    let mut sys_info = SystemInfo::new();

    get_cpu_info(&mut sys_info.cpu);

    get_gpu_info(&mut sys_info.gpu);

    get_kernel_info(&mut sys_info.kernel);

    get_os(&mut sys_info.os);

    println!("{:?}", sys_info);
}
