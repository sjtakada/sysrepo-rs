//
// Sysrepo-examples.
//   oper_data
//

use std::env;
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
    log_stderr(SrLogLevel::Warn);

    // Connect to sysrepo.
    let mut sr = match SrConn::new(0) {
        Ok(sr) => sr,
        Err(_) => return false,
    };

    // Callback
    let f = |ctx: &LibYangCtx,
             sub_id: u32,
             mod_name: &str,
             path: &str,
             _request_xpath: Option<&str>,
             _request_id: u32|
     -> Option<LydNode> {
        println!("");
        println!("");
        println!(
            r#" ========== DATA ({}) FOR "{}" "{}" REQUESED ======================="#,
            sub_id, mod_name, path
        );
        println!("");

        if mod_name == "examples" && path == "/examples:stats" {
            let path1 = String::from("/examples:stats/counter");
            let val1 = LydValue::from_string("852".to_string());
            let path2 = String::from("/examples:stats/counter2");
            let val2 = LydValue::from_string("1052".to_string());

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
    if let Err(_) = sess.oper_get_subscribe(&mod_name, &path, f, 0) {
        return false;
    }

    println!("\n\n ========== LISTENING FOR REQUESTS ==========\n");

    signal_init();
    while !is_sigint_caught() {
        thread::sleep(time::Duration::from_secs(1));
    }

    true
}
