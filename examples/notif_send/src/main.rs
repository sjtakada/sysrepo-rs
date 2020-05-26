//
// Sysrepo-examples.
//   notif_send
//

use std::env;
use std::ffi::c_void;
use std::mem::zeroed;

use sysrepo::*;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <notification-path> [<node-to-set> <node-value>]",
        program
    );
}

/// Main.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 2 || args.len() > 4 || args.len() == 3 {
        print_help(&program);
        std::process::exit(1);
    }

    let path = args[1].clone();
    let node_path_val = if args.len() == 4 {
        Some((args[2].clone(), args[3].clone()))
    } else {
        None
    };

    let mut conn: *mut sr_conn_ctx_t = unsafe { zeroed::<*mut sr_conn_ctx_t>() };
    let mut session: *mut sr_session_ctx_t = unsafe { zeroed::<*mut sr_session_ctx_t>() };
    let mut rc;

    println!(
        r#"Application will send notification "{}" notification."#,
        path
    );

    // Turn logging on.
    unsafe {
        sr_log_stderr(sr_log_level_t_SR_LL_WRN);
    }

    loop {
        // Generic raw null pointer.
        let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        // Connect to sysrepo.
        let ctx;
        unsafe {
            rc = sr_connect(0, &mut conn);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
            ctx = sr_get_context(conn);
        }

        // Start session.
        unsafe {
            rc = sr_session_start(conn, sr_datastore_e_SR_DS_RUNNING, &mut session);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        // Create the notification.
        let notif;
        unsafe {
            let path_ptr = &path[..] as *const _ as *const i8;
            notif = lyd_new_path(null_ptr as *mut lyd_node, ctx, path_ptr, null_ptr, 0, 0);
            if notif == null_ptr as *mut lyd_node {
                println!(r#"Creating notification "{}" failed."#, path);
                break;
            }
        }

        // Add the input value.
        unsafe {
            if let Some((path, val)) = node_path_val {
                let path_ptr = &path[..] as *const _ as *const i8;
                let val_ptr = &val[..] as *const _ as *mut c_void;

                let ret_node =
                    lyd_new_path(notif, null_ptr as *mut ly_ctx, path_ptr, val_ptr, 0, 0);
                if ret_node == null_ptr as *mut lyd_node {
                    println!(r#"Creating value "{}" failed."#, path);
                    break;
                }
            }
        }

        // Send the notification.
        unsafe {
            rc = sr_event_notif_send_tree(session, notif);
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
