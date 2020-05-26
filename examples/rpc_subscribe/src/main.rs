//
// Sysrepo-examples.
//   rpc_subscribe
//

use std::env;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::mem::zeroed;
use std::os::raw::c_char;
use std::thread;
use std::time;

use libc;
use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <path-to-rpc>", program);
}

/// Notification callback.
extern "C" fn rpc_cb(
    _session: *mut sr_session_ctx_t,
    path: *const c_char,
    input: *const sr_val_t,
    input_cnt: u64,
    _event: sr_event_t,
    _request_id: u32,
    output: *mut *mut sr_val_t,
    output_cnt: *mut u64,
    _private_data: *mut ::std::os::raw::c_void,
) -> i32 {
    let path: &CStr = unsafe { CStr::from_ptr(path) };
    println!("");
    println!("");
    println!(
        r#" ========== RPC "{}" RECEIVED ======================="#,
        path.to_str().unwrap()
    );
    println!("");

    if path.to_str().unwrap() == "/examples:oper" {
        unsafe {
            let xpath = CString::new("/examples:oper/ret").unwrap();
            let xpath_ptr = xpath.as_ptr();
            let data = libc::malloc(mem::size_of::<sr_val_t>()) as *mut sr_val_t;

            (*data).xpath = libc::strdup(xpath_ptr);
            (*data).type_ = sr_type_e_SR_INT64_T;
            (*data).dflt = false;
            (*data).data.int64_val = -123456i64;

            *output_cnt = 1;
            *output = data;
        }
    }

    sr_error_e_SR_ERR_OK as i32
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
    let mut subscription: *mut sr_subscription_ctx_t =
        unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
    let mut rc;

    println!(r#"Application will subscribe "{}" RPC."#, path);

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

        // Subscribe for the RPC.
        unsafe {
            let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let path = &path[..] as *const _ as *const i8;

            rc = sr_rpc_subscribe(
                session,
                path,
                Some(rpc_cb),
                null_ptr,
                0,
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
