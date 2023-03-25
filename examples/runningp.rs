// process listing
use std::sync::Arc;

use sysinfo::{System, ProcessExt, SystemExt};

fn main() {
    let s = System::new_all();

    for process in s.processes() {
        println!("{:?}, {}",  process.0, process.1.name())
    }

}

