//
// Sysrepo-examples.
//   sr_get_items
//

use std::env;
use std::slice;
use std::mem::zeroed;

use utils::print_val;
use sysrepo::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <x-path-to-get> [running/operational]", program);
}

/// Main.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 2 && args.len() != 3 {
        print_help(&program);
        std::process::exit(1);
    }

    let mut conn: *mut sr_conn_ctx_t = unsafe { zeroed::<*mut sr_conn_ctx_t>() };
    let mut session: *mut sr_session_ctx_t = unsafe { zeroed::<*mut sr_session_ctx_t>() };
    let mut rc;

    let xpath = args[1].clone();
    let mut ds = sr_datastore_e_SR_DS_RUNNING;

    if args.len() == 3 {
        if args[2] == "running" {
            ds = sr_datastore_e_SR_DS_RUNNING;
        } else if args[2] == "operational" {
            ds = sr_datastore_e_SR_DS_OPERATIONAL;
        } else {
            println!("Invalid datastore {}.", args[2]);
            std::process::exit(1);
        }
    }

    println!(r#"Application will get "{}" from "{}" datastore."#,
             xpath, if ds == sr_datastore_e_SR_DS_RUNNING { "running" } else { "operational" });

    // Turn logging on.
    unsafe {
        sr_log_stderr(sr_log_level_t_SR_LL_WRN);
    }

    loop {
        // Connect to sysrepo.
        unsafe {
            rc = sr_connect(0, &mut conn);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        // Start session.
        unsafe {
            rc = sr_session_start(conn, ds, &mut session);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        let mut val_count: u64 = 0;
        let mut vals: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };

        // Get the values.
        unsafe {
            let xpath = &xpath[..] as *const _ as *const i8;

            rc = sr_get_items(session, xpath, 0, 0, &mut vals, &mut val_count);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        // Print the values.
        unsafe {
            let vals: &[sr_val_t] = slice::from_raw_parts(vals, val_count as usize);

            for i in 0..vals.len() {
                print_val(&vals[i]);
            }
        }

        break;
    }

    unsafe {
        sr_disconnect(conn);
    }

    if rc == 0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
