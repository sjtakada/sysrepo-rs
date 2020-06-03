//
// Sysrepo-examples.
//   rpc_send
//

use std::env;

use sysrepo::*;
use utils::print_val;

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

    if args.len() != 2 {
        print_help(&program);
        return false;
    }

    let path = args[1].clone();

    println!(r#"Application will send RPC "{}" notification."#, path);

    // Turn logging on.
    Sysrepo::log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Start session.
    let sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };


    // Send the RPC.
    match sess.rpc_send(&path, None, None) {
        Ok(vec) => {
            for v in vec {
                print_val(&v);
            }
        }
        Err(_) => return false,
    };


/*
    let mut output_count: u64 = 0;
    let mut output: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };

    // Send the RPC.
    unsafe {
        let path = &path[..] as *const _ as *const i8;

        rc = sr_rpc_send(
            session,
            path,
            null_ptr as *const sr_val_t,
            0,
            0,
            &mut output,
            &mut output_count,
        );
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
*/

    true
}
