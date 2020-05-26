//
// Sysrepo-examples.
//   oper_data
//

use std::env;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem::zeroed;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::thread;
use std::time;

use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <module-to-provide-data-from> <path-to-provide>",
        program
    );
}

/// Notification callback.
extern "C" fn dg_get_items_cb(
    session: *mut sr_session_ctx_t,
    module_name: *const c_char,
    xpath: *const c_char,
    _request_xpath: *const c_char,
    _request_id: u32,
    parent: *mut *mut lyd_node,
    _private_data: *mut c_void,
) -> i32 {
    let xpath: &CStr = unsafe { CStr::from_ptr(xpath) };
    let module_name: &CStr = unsafe { CStr::from_ptr(module_name) };

    println!("");
    println!("");
    println!(
        r#" ========== DATA FOR "{}" "{}" REQUESED ======================="#,
        module_name.to_str().unwrap(),
        xpath.to_str().unwrap()
    );
    println!("");

    if module_name.to_str().unwrap() == "examples" && xpath.to_str().unwrap() == "/examples:stats" {
        let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let path1 = CString::new("/examples:stats/counter").unwrap();
        let val1 = CString::new("852").unwrap();
        let path2 = CString::new("/examples:stats/counter2").unwrap();
        let val2 = CString::new("1052").unwrap();

        unsafe {
            let context = sr_get_context(sr_session_get_connection(session));

            *parent = lyd_new_path(
                null_ptr as *mut lyd_node,
                context,
                path1.as_ptr() as *const _ as *const i8,
                val1.as_ptr() as *const _ as *mut c_void,
                0,
                0,
            );
            lyd_new_path(
                *parent,
                null_ptr as *const ly_ctx,
                path2.as_ptr() as *const _ as *const i8,
                val2.as_ptr() as *const _ as *mut c_void,
                0,
                0,
            );
        }
    }

    sr_error_e_SR_ERR_OK as i32
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
    let mut subscription: *mut sr_subscription_ctx_t =
        unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
    let mut rc;

    let mod_name = args[1].clone();
    let path = args[2].clone();

    println!(
        r#"Application will provide data "{}" of "{}"."#,
        path, mod_name
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

        // Subscribe for the providing the operational data.
        unsafe {
            let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
            let mod_name = &mod_name[..] as *const _ as *const i8;
            let path = &path[..] as *const _ as *const i8;

            rc = sr_oper_get_items_subscribe(
                session,
                mod_name,
                path,
                Some(dg_get_items_cb),
                null_ptr,
                0,
                &mut subscription,
            );
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        println!("\n\n ========== LISTENING FOR REQUESTS ==========\n");

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
