use std::fs::File;
use std::io::{BufRead, BufReader};

struct SystemInfo {
    cpu: Option<String>,
}

impl SystemInfo {
    fn new() -> Self {
        SystemInfo { cpu: None }
    }
}

fn main() {
    let mut sys_info = SystemInfo::new();
    // read in proc/cpuinfo
    let proc_file = File::open("/proc/cpuinfo").unwrap();

    let reader = BufReader::new(proc_file);

    // read until
    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("model name") {
            // split the string on the ':', get the second item
            let model_name_set: Vec<&str> = line.split(":").collect();

            // get the string after the : if it exists
            // check that the vector is at least 2 long
            if model_name_set.len() >= 2 {
                println!("{}", model_name_set[1].trim());

                sys_info.cpu = Some(String::from(model_name_set[1].trim()));
            }

            break;
        }
    }
}
