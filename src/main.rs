use std::fs;

fn main() {
    // read in proc/cpuinfo
    let proc_contents = fs::read_to_string("/proc/cpuinfo").unwrap();

    // print it
    println!("{}", proc_contents);
}
