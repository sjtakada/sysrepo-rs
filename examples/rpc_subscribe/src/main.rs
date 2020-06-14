//
// Sysrepo-examples.
//   rpc_subscribe
//

use std::env;
use std::thread;
use std::time;

use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <path-to-rpc>", program);
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

    println!(r#"Application will subscribe "{}" RPC."#, path);

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

    // Callback function.
    let f = |_sess: SrSession, _op_path: &str, _inputs: SrValueSlice,
             _event: SrEvent, _request_id: u32| -> SrValueSlice
    {
        let mut sr_output = SrValueSlice::new(1, false);
        sr_output.set_int64_value(0, false, "/examples:oper/ret", -123456);
        sr_output
    };

    // Subscribe for the RPC.
    if let Err(_) = sess.rpc_subscribe(Some(path), f, 0, 0) {
        return false;
    }

    println!("\n\n ========== LISTENING FOR NOTIFICATIONS ==========\n");

    signal_init();
    while !is_sigint_caught() {
        thread::sleep(time::Duration::from_secs(1));
    }

    println!("Application exit requested, exiting.");

    true
}
