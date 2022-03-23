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

    // Turn logging on.
    log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match SrConn::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Start session.
    let sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    // Set the value.
    if let Err(_) = sess.set_item_str(&xpath, &value, None, 0) {
        return false;
    }

    // Apply the change.
    if let Err(_) = sess.apply_changes(None) {
        return false;
    }

    true
}
