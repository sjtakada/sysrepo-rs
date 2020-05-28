//
// Sysrepo-examples.
//   notif_subscribe
//

use std::env;
use std::ffi::CStr;
use std::mem::zeroed;
use std::os::raw::c_char;
use std::slice;
use std::thread;
use std::time;
use std::os::raw::c_void;

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

    Sysrepo::log_stderr(SrLogLevel::Warn);

    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    let mut sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    let f = |sess: *mut sr_session_ctx_t, notif_type:sr_ev_notif_type_t,
             path: *const c_char, values: *const sr_val_t, values_cnt: size_t,
             timestamp: time_t|
    {
        let path: &CStr = unsafe { CStr::from_ptr(path) };
        println!("");
        println!("");
        println!(
            r#" ========== NOTIFICATION "{}" RECEIVED ======================="#,
            path.to_str().unwrap()
        );
        println!("");

        unsafe {
            let vals: &[sr_val_t] = slice::from_raw_parts(values, values_cnt as usize);

            for i in 0..vals.len() {
                print_val(&vals[i]);
            }
        }
    };

    if let Err(_) = sess.event_notif_subscribe(&mod_name, xpath, None, None, f,
                                               std::ptr::null_mut(), 0) {
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
