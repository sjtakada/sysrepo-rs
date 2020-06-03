//
// Sysrepo-examples.
//   rpc_subscribe
//

use std::env;
use std::ffi::CString;
use std::thread;
use std::time;

use libc;
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

    // Callback function.
    let f = |_id: u32, _op_path: &str, _inputs: SysrepoValues,
             _event: sr_event_t, _request_id: u32| -> Vec<sr_val_t>
    {
        let mut vec = Vec::new();

        let xpath = CString::new("/examples:oper/ret").unwrap();
        let xpath_ptr = xpath.as_ptr();
        unsafe {
            let mut val: sr_val_t = std::mem::zeroed::<sr_val_t>();
            val.xpath = libc::strdup(xpath_ptr);
            val.type_ = sr_type_e_SR_INT64_T;
            val.dflt = false;
            val.data.int64_val = -123456164;

            vec.push(val);
        }

        /*
        let xpath = CString::new("/examples:oper/ret2").unwrap();
        let xpath_ptr = xpath.as_ptr();
        unsafe {
            let mut val: sr_val_t = std::mem::zeroed::<sr_val_t>();
            val.xpath = libc::strdup(xpath_ptr);
            val.type_ = sr_type_e_SR_INT64_T;
            val.dflt = false;
            val.data.int64_val = 123456789;

            vec.push(val);
        }
        */

        vec
    };

    // Subscribe for the RPC.
    if let Err(_) = sess.rpc_subscribe(Some(path), f, std::ptr::null_mut(), 0, 0) {
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
