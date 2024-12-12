use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct SystemInfo {
    cpu: Option<String>,
    cores: Option<String>,
    cpu_frequency_in_hz: Option<u32>,
}

impl SystemInfo {
    fn new() -> Self {
        SystemInfo {
            cpu: None,
            cores: None,
            cpu_frequency_in_hz: None,
        }
    }
}

/// Gets the second item in the line after the ':' and trims it accordingly
fn store_proc_cpuinfo(pointer: &mut Option<String>, line: &String) {
    // split the string on the ':', get the second item
    let slices: Vec<&str> = line.split(":").collect();

    // get the string after the : if it exists
    // check that the vector is at least 2 long
    if slices.len() >= 2 {
        *pointer = Some(String::from(slices[1].trim()));
    }
}

/// gather the cpu info from /proc/cpuinfo
/// and from /sys/devices/system/cpu/cpu0/cpufreq/bios_limit
///
/// cpuinfo gives cpu_name and how many cores
/// cpufreq gives the frequency in hz
fn get_cpu_info(
    cpu_name: &mut Option<String>,
    cores: &mut Option<String>,
    cpu_freq_in_hz: &mut Option<u32>,
) {
    let proc_file = File::open("/proc/cpuinfo").unwrap();
    let reader = BufReader::new(proc_file);

    let mut model_name_found: bool = false;
    let mut cpu_cores_found: bool = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if !model_name_found && line.contains("model name") {
            store_proc_cpuinfo(cpu_name, &line);
            model_name_found = true;
        } else if !cpu_cores_found && line.contains("cpu cores") {
            store_proc_cpuinfo(cores, &line);
            cpu_cores_found = true;
        }

        // break out so we dont waste time looking
        if cpu_cores_found && model_name_found {
            break;
        }
    }

    let cpu_freq_file_as_string =
        fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/bios_limit").unwrap();

    // get the string, trim and parse it into a u32
    *cpu_freq_in_hz = Some(cpu_freq_file_as_string.trim().parse::<u32>().unwrap());
}

fn main() {
    let mut sys_info = SystemInfo::new();

    get_cpu_info(
        &mut sys_info.cpu,
        &mut sys_info.cores,
        &mut sys_info.cpu_frequency_in_hz,
    );

    println!("{:?}", sys_info);
}
