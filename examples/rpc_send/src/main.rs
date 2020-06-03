//
// Sysrepo-examples.
//   rpc_send
//

use std::env;

use sysrepo::*;
use utils::print_val;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <notification-path> [<node-to-set> <node-value>]",
        program
    );
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

    if args.len() != 2 {
        print_help(&program);
        return false;
    }

    let path = args[1].clone();

    println!(r#"Application will send RPC "{}" notification."#, path);

    // Turn logging on.
    Sysrepo::log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Start session.
    let sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    // Send the RPC.
    match sess.rpc_send(&path, None, None) {
        Ok(mut sr_values) => {
            for v in sr_values.as_slice() {
                print_val(&v);
            }
        }
        Err(_) => return false,
    };

    true
}
