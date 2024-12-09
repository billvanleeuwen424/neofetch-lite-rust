use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    // read in proc/cpuinfo
    let proc_file = File::open("/proc/cpuinfo").unwrap();

    let reader = BufReader::new(proc_file);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains("model name") {
            // split the string on the ':', get the second item
            let model_name_set: Vec<&str> = line.split(":").collect();

            // get the string after the : if it exists
            // check that the vector is at least 2 long
            if model_name_set.len() >= 2 {
                println!("{}", model_name_set[1].trim());
            }

            break;
        }
    }
}
