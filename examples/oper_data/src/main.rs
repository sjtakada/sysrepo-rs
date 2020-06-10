//
// Sysrepo-examples.
//   oper_data
//

use std::env;
use std::thread;
use std::time;
use std::sync::Arc;

use std::ffi::CString;

use sysrepo::*;
use utils::*;

/// Show help.
fn print_help(program: &str) {
    println!(
        "Usage: {} <module-to-provide-data-from> <path-to-provide>",
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

    if args.len() != 3 {
        print_help(&program);
        std::process::exit(1);
    }

    let mod_name = args[1].clone();
    let path = args[2].clone();

    println!(
        r#"Application will provide data "{}" of "{}"."#,
        path, mod_name
    );

    // Turn logging on.
    Sysrepo::log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match Sysrepo::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Get Lib Yang Context from sysrepo connection.
    //let mut ly_ctx = Arc::new(sr.get_context());

    // Callback
    let f = |ctx: &LibYangCtx, mod_name: &str,
             path: &str, _request_xpath: Option<&str>,
             _request_id: u32| -> Option<LydNode>
    {
//        let ctx = Arc::get_mut(&mut ly_ctx).unwrap();

        println!("");
        println!("");
        println!(
            r#" ========== DATA FOR "{}" "{}" REQUESED ======================="#,
            mod_name, path
        );
        println!("");

        if mod_name == "examples" && path == "/examples:stats" {
            let path1 = CString::new("/examples:stats/counter").unwrap();
            let val1 = LydValue::from_str("852");
            let path2 = CString::new("/examples:stats/counter2").unwrap();
            let val2 = LydValue::from_str("1052");

            let parent = LibYang::lyd_new_path(None, Some(ctx), &path1, Some(&val1), 0).unwrap();
            LibYang::lyd_new_path(Some(&parent), None, &path2, Some(&val2), 0);

            Some(parent)

        } else {
            None
        }
    };

    // Start session.
    let sess = match sr.start_session(SrDatastore::Running) {
        Ok(sess) => sess,
        Err(_) => return false,
    };

    // Subscribe for the providing the operational data.
    let subscr = match sess.oper_get_items_subscribe(&mod_name, &path, f, 0) {
        Ok(subscr) => subscr,
        Err(_) => return false,
    };

    println!("\n\n ========== LISTENING FOR REQUESTS ==========\n");

    signal_init();
    while !is_sigint_caught() {
        thread::sleep(time::Duration::from_secs(1));
    }

    true
}
