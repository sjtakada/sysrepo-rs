//
// Sysrepo-examples.
//   application_changes
//

use std::env;
use std::slice;
use std::mem::zeroed;
use std::os::raw::c_void;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::time;
use std::thread;

use utils::*;
use sysrepo::*;

/// Show help.
fn print_help(program: &str) {
    println!("Usage: {} <module-to-subscribe> [<xpath-to-subscribe>]", program);
}

/// Print change.
fn print_change(op: sr_change_oper_t, old_val: *mut sr_val_t, new_val: *mut sr_val_t) {
    unsafe {
        let old_val: &sr_val_t = &*old_val;
        let new_val: &sr_val_t = &*new_val;

        match op {
            sr_change_oper_e_SR_OP_CREATED => {
                print!("CREATED: ");
                print_val(new_val);
            }
            sr_change_oper_e_SR_OP_DELETED => {
                print!("DELETED: ");
                print_val(old_val);
            }
            sr_change_oper_e_SR_OP_MODIFIED => {
                print!("MODIFIED: ");
                print_val(old_val);
                print!("to ");
                print_val(new_val);
            }
            sr_change_oper_e_SR_OP_MOVED => {
                let xpath = unsafe { CStr::from_ptr(new_val.xpath) };
                println!("MOVED: {}", xpath.to_str().unwrap());
            }
            _ => {
            }
        }
    }
}

/// Print current config.
fn print_current_config(session: *mut sr_session_ctx_t, module_name: &str) {
    let mut values: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
    let mut count: u64 = 0;
    let rc;
    let xpath = format!("/{}:*//.", module_name);

    let xpath = &xpath[..] as *const _ as *const i8;
    unsafe {
        rc = sr_get_items(session, xpath, 0, 0, &mut values, &mut count);
        if rc != sr_error_e_SR_ERR_OK as i32 {
            return;
        }
    }

    unsafe {
        let vals: &[sr_val_t] = slice::from_raw_parts(values, count as usize);

        for i in 0..vals.len() {
            print_val(&vals[i]);
        }
    }
}

/// Module change callback.
extern "C" fn module_change_cb(session: *mut sr_session_ctx_t,
                               module_name: *const c_char,
                               _xpath: *const c_char,
                               event: sr_event_t,
                               _request_id: u32,
                               _private_data: *mut c_void) -> i32 {

    let mut it: *mut sr_change_iter_t = unsafe { zeroed::<*mut sr_change_iter_t>() };
    let mut old_value: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
    let mut new_value: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
    let mut oper: sr_change_oper_t = 0;
    
    let rc;

    println!("");
    println!("");
    println!(" ========== EVENT {} CHANGES: ====================================", event);
    println!("");

    loop {
        unsafe {
            let path = CString::new("//.").unwrap();

            rc = sr_get_changes_iter(session, path.as_ptr() as *const _ as *const i8, &mut it);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        unsafe {
            while sr_get_change_next(session, it, &mut oper, &mut old_value, &mut new_value) == sr_error_e_SR_ERR_OK as i32 {
                print_change(oper, old_value, new_value);
                sr_free_val(old_value);
                sr_free_val(new_value);
            }
        }

        println!("");
        print!(" ========== END OF CHANGES =======================================");

        if event == sr_event_e_SR_EV_DONE {
            let module_name = unsafe { CStr::from_ptr(module_name) };

            println!("");
            println!("");
            println!(" ========== CONFIG HAS CHANGED, CURRENT RUNNING CONFIG: ==========");
            println!("");
            print_current_config(session, module_name.to_str().unwrap());
        }

        break;
    }

    unsafe {
        sr_free_change_iter(it);
    }

    sr_error_e_SR_ERR_OK as i32
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
    let mut subscription: *mut sr_subscription_ctx_t = unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
    let mut rc;

    let mod_name = args[1].clone();

    println!(r#"Application will watch for changes in "{}"."#,
             if args.len() == 3 { args[2].clone() } else { args[1].clone() });

    // Turn logging on.
    unsafe {
        sr_log_stderr(sr_log_level_t_SR_LL_WRN);
    }

    loop {
        let null_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

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

        // Read current config.
        println!("");
        println!(" ========== READING RUNNING CONFIG: ==========");
        println!("");
        print_current_config(session, &mod_name);

        // Subscribe for changes in running config.
        unsafe {
            let mod_name = &mod_name[..] as *const _ as *const i8;
            let xpath = if args.len() == 3 {
                let xpath = args[2].clone();
                &xpath[..] as *const _ as *mut i8
            } else {
                null_ptr as *const i8
            };

            rc = sr_module_change_subscribe(session, mod_name, xpath,
                                            Some(module_change_cb),
                                            null_ptr, 0, 0, &mut subscription);
            if rc != sr_error_e_SR_ERR_OK as i32 {
                break;
            }
        }

        println!("\n\n ========== LISTENING FOR CHANGES ==========\n");

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
