//
// Sysrepo-examples.
//   application_changes
//

use std::env;
use std::ffi::CStr;
//use std::ffi::CString;
//use std::mem::zeroed;
//use std::os::raw::c_char;
//use std::os::raw::c_void;
//use std::slice;
use std::thread;
use std::time;

use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <module-to-subscribe> [<xpath-to-subscribe>]",
        program
    );
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
                let xpath = CStr::from_ptr(new_val.xpath);
                println!("MOVED: {}", xpath.to_str().unwrap());
            }
            _ => {}
        }
    }
}

/// Print current config.
fn print_current_config(sess: &mut SysrepoSession, mod_name: &str) {
    let xpath = format!("/{}:*//.", mod_name);
    let xpath = &xpath[..];// as *const _ as *const i8;

    // Get the values.
    match sess.get_items(&xpath, None, 0) {
        Err(_) => {
        }
        Ok(mut values) => {
            for v in values.as_slice() {
                print_val(&v);
            }
        }
    }
/*
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
*/
}

/*
/// Module change callback.
extern "C" fn module_change_cb(
    session: *mut sr_session_ctx_t,
    module_name: *const c_char,
    _xpath: *const c_char,
    event: sr_event_t,
    _request_id: u32,
    _private_data: *mut c_void,
) -> i32 {
    let mut it: *mut sr_change_iter_t = unsafe { zeroed::<*mut sr_change_iter_t>() };
    let mut old_value: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
    let mut new_value: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
    let mut oper: sr_change_oper_t = 0;

    let rc;

    println!("");
    println!("");
    println!(
        " ========== EVENT {} CHANGES: ====================================",
        event
    );
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
            while sr_get_change_next(session, it, &mut oper, &mut old_value, &mut new_value)
                == sr_error_e_SR_ERR_OK as i32
            {
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
//            print_current_config(session, module_name.to_str().unwrap());
        }

        break;
    }

    unsafe {
        sr_free_change_iter(it);
    }

    sr_error_e_SR_ERR_OK as i32
}
*/

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

    if args.len() != 2 && args.len() != 3 {
        print_help(&program);
        std::process::exit(1);
    }

    let mod_name = args[1].clone();

    println!(
        r#"Application will watch for changes in "{}"."#,
        if args.len() == 3 {
            args[2].clone()
        } else {
            args[1].clone()
        }
    );

    // Turn logging on.
    Sysrepo::log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Start session.
    let mut sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    // Read current config.
    println!("");
    println!(" ========== READING RUNNING CONFIG: ==========");
    println!("");
    print_current_config(&mut sess, &mod_name);


    let xpath = args[2].clone();
    let xpath = if args.len() == 3 {
        Some(&xpath[..])
    } else {
        None
    };

    let sess_clone = sess.clone();

    let f = move |_id: u32, mod_name: &str, path: &str, event: sr_event_t, _request_id: u32| -> ()
    {
        let path = "//.";

        let it = match sess_clone.get_changes_iter(&path) {
            Err(_) => return,
            Ok(it) => it,
        };
    };

    // Subscribe for changes in running config.
    if let Err(_) = sess.module_change_subscribe(&mod_name, xpath, f, 0, 0) {
        return false;
    }

    println!("\n\n ========== LISTENING FOR CHANGES ==========\n");

    signal_init();
    while !is_sigint_caught() {
        thread::sleep(time::Duration::from_secs(1));
    }

    println!("Application exit requested, exiting.");

    true
}
