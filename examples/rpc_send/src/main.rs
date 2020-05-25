//
// Sysrepo-examples.
//   rpc_send
//

use std::env;
use std::slice;
use std::mem::zeroed;
use std::ffi::c_void;

use utils::print_val;
use sysrepo::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <notification-path> [<node-to-set> <node-value>]", program);
}

/// Main.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 2 {
        print_help(&program);
        std::process::exit(1);
    }

    let path = args[1].clone();

    let mut conn: *mut sr_conn_ctx_t = unsafe { zeroed::<*mut sr_conn_ctx_t>() };
    let mut session: *mut sr_session_ctx_t = unsafe { zeroed::<*mut sr_session_ctx_t>() };
    let mut rc;

    println!(r#"Application will send RPC "{}" notification."#, path);

    // Turn logging on.
    unsafe {
        sr_log_stderr(sr_log_level_t_SR_LL_WRN);
    }

    loop {
        // Generic raw null pointer.
        let null_ptr: *mut c_void = std::ptr::null_mut();

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

        let mut output_count: u64 = 0;
        let mut output: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };

        // Send the RPC.
        unsafe {
            let path = &path[..] as *const _ as *const i8;

            rc = sr_rpc_send(session, path, null_ptr as *const sr_val_t, 0, 0,
                             &mut output, &mut output_count);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        // Print the values.
        unsafe {
            let vals: &[sr_val_t] = slice::from_raw_parts(output, output_count as usize);

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
