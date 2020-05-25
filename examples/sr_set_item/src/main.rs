//
// Sysrepo-examples.
//   sr_set_item
//

use std::env;
use std::mem::zeroed;

use sysrepo::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <x-path-to-set> <value-to-set>", program);
}

/// Main.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 3 {
        print_help(&program);
        std::process::exit(1);
    }

    let mut conn: *mut sr_conn_ctx_t = unsafe { zeroed::<*mut sr_conn_ctx_t>() };
    let mut session: *mut sr_session_ctx_t = unsafe { zeroed::<*mut sr_session_ctx_t>() };
    let mut rc;

    let xpath = args[1].clone();
    let value = args[2].clone();

    println!(r#"Application will get "{}" to "{}"."#, xpath, value);

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
            rc = sr_session_start(conn, sr_datastore_e_SR_DS_RUNNING, &mut session);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        // Set the value.
        unsafe {
            let xpath = &xpath[..] as *const _ as *const i8;
            let value = &value[..] as *const _ as *const i8;

            rc = sr_set_item_str(session, xpath, value, std::ptr::null(), 0);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        // Apply change.
        unsafe {
            rc = sr_apply_changes(session, 0, 0);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
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
