//
// Sysrepo-examples.
//   notif_subscribe
//

use std::env;
use std::thread;
use std::time;

use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <module-with-notification> [<xpath-filtering-notifications>]",
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

    if args.len() != 2 && args.len() != 3 {
        print_help(&program);
        return false;
    }

    let mod_name = args[1].clone();
    let xpath = if args.len() == 3 {
        Some(args[2].clone())
    } else {
        None
    };

    println!(
        r#"Application will subscribe "{}" notifications."#,
        mod_name
    );

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
    let f = |_sess: SrSession, _notif_type: SrNotifType,
             path: &str, mut values: SrValueSlice, _timestamp: time_t|
    {
        println!("");
        println!("");
        println!(
            r#" ========== NOTIFICATION "{}" RECEIVED ======================="#, path
        );
        println!("");

        for v in values.as_slice() {
            print_val(&v);
        }
    };

    // Subscribe for the notifications.
    if let Err(_) = sess.event_notif_subscribe(&mod_name, xpath, None, None, f,
                                               std::ptr::null_mut(), 0) {
        return false;
    }

    println!("\n\n ========== LISTENING FOR NOTIFICATIONS ==========\n");

    // Loop until ctrl-c is pressed / SIGINT is received.
    signal_init();
    while !is_sigint_caught() {
        thread::sleep(time::Duration::from_secs(1));
    }

    println!("Application exit requested, exiting.");

    true
}
