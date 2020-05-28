//
// Sysrepo-examples.
//   sr_set_item
//

use std::env;

use sysrepo::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <x-path-to-set> <value-to-set>", program);
}

/// Main.
fn main() {
    if run() {
        std::process::exit(0);
    } else {
        std::process::exit(0);
    }
}

fn run() -> bool {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 3 {
        print_help(&program);
        return false;
    }

    let xpath = args[1].clone();
    let value = args[2].clone();

    println!(r#"Application will get "{}" to "{}"."#, xpath, value);

    Sysrepo::log_stderr(SrLogLevel::Warn);

    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    let mut sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    if let Err(_) = sess.set_item_str(&xpath, &value, None, 0) {
        return false;
    }

    if let Err(_) = sess.apply_changes(None, false) {
        return false;
    }

    true
}
