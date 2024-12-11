use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct SystemInfo {
    cpu: Option<String>,
    cores: Option<String>,
}

impl SystemInfo {
    fn new() -> Self {
        SystemInfo {
            cpu: None,
            cores: None,
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

// gather the cpu info from /proc/cpuinfo
fn get_cpu_info(cpu_name: &mut Option<String>, cores: &mut Option<String>) {
    let proc_file = File::open("/proc/cpuinfo").unwrap();
    let reader = BufReader::new(proc_file);

    for line in reader.lines() {
        let line = line.unwrap();

        if line.contains("model name") {
            store_proc_cpuinfo(cpu_name, &line);
        } else if line.contains("cpu cores") {
            store_proc_cpuinfo(cores, &line);
        }
    }
}

fn main() {
    let mut sys_info = SystemInfo::new();

    get_cpu_info(&mut sys_info.cpu, &mut sys_info.cores);
    println!("{:?}", sys_info);
}
