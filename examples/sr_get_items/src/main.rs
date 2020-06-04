//
// Sysrepo-examples.
//   sr_get_items
//

use std::env;

use sysrepo::*;
use utils::print_val;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <x-path-to-get> [running/operational]", program);
}

/// Main.
fn main() {
    if run() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn run() -> bool {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 2 && args.len() != 3 {
        print_help(&program);
        return false;
    }

    let xpath = args[1].clone();
    let mut ds = SrDatastore::Running;

    if args.len() == 3 {
        if args[2] == "running" {
            ds = SrDatastore::Running;
        } else if args[2] == "operational" {
            ds = SrDatastore::Operational;
        } else {
            println!("Invalid datastore {}.", args[2]);
            return false;
        }
    }

    println!(
        r#"Application will get "{}" from "{}" datastore."#,
        xpath,
        if ds == SrDatastore::Running {
            "running"
        } else {
            "operational"
        }
    );

    // Turn logging on.
    Sysrepo::log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Start session.
    let sess = match sr.start_session(ds) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    // Get the values.
    match sess.get_items(&xpath, None, 0) {
        Err(_) => return false,
        Ok(mut values) => {
            for v in values.as_slice() {
                print_val(&v);
            }
        }
    }

    true
}
