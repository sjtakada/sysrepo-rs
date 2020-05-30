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
    if run() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn run() -> bool {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 2 || args.len() > 4 || args.len() == 3 {
        print_help(&program);
        return false;
    }

    let path = args[1].clone();
    let node_path_val = if args.len() == 4 {
        Some((args[2].clone(), args[3].clone()))
    } else {
        None
    };

//    let mut conn: *mut sr_conn_ctx_t = unsafe { zeroed::<*mut sr_conn_ctx_t>() };
//    let mut session: *mut sr_session_ctx_t = unsafe { zeroed::<*mut sr_session_ctx_t>() };
//    let mut rc;

    println!(
        r#"Application will send notification "{}" notification."#,
        path
    );

    // Turn logging on.
    Sysrepo::log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    //ctx = sr_get_context(conn);

    // Start session.
    let sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    // Create the notification.
/*
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

*/

    true
}
