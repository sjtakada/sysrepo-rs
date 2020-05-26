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

use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <module-with-notification> [<xpath-filtering-notifications>]",
        program
    );
}

/// Notification callback.
extern "C" fn notif_cb(
    _session: *mut sr_session_ctx_t,
    _notif_type: sr_ev_notif_type_t,
    path: *const c_char,
    values: *const sr_val_t,
    values_cnt: u64,
    _timestamp: time_t,
    _private_data: *mut ::std::os::raw::c_void,
) {
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
    let mut subscription: *mut sr_subscription_ctx_t =
        unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
    let mut rc;

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

        // Subscribe for the notifications.
        unsafe {
            let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let mod_name = &mod_name[..] as *const _ as *const i8;
            let xpath = match xpath {
                Some(xpath) => &xpath[..] as *const _ as *const i8,
                None => null_ptr as *const i8,
            };

            rc = sr_event_notif_subscribe(
                session,
                mod_name,
                xpath,
                0,
                0,
                Some(notif_cb),
                null_ptr,
                0,
                &mut subscription,
            );
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        println!("\n\n ========== LISTENING FOR NOTIFICATIONS ==========\n");

        signal_init();
        while !is_sigint_caught() {
            thread::sleep(time::Duration::from_secs(1));
        }

        println!("Application exit requested, exiting.");

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
