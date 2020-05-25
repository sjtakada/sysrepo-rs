//
// Sysrepo-examples.
//   utils
//

//#![warn(non_upper_case_globals)]

use std::ffi::CStr;
use std::sync;

use nix::sys::signal;

use sysrepo::*;

/// Print value.
pub fn print_val(value: &sr_val_t) {
    let sr_val_p: *const sr_val_t = value as *const sr_val_t;
    if sr_val_p.is_null() {
        return
    }

    let xpath: &CStr = unsafe { CStr::from_ptr(value.xpath) };

    print!("{} ", xpath.to_str().unwrap());

    let v = match value.type_ {
        sr_type_e_SR_CONTAINER_T |
        sr_type_e_SR_CONTAINER_PRESENCE_T => String::from("(container)"),
        sr_type_e_SR_LIST_T => String::from("(list instance)"),
        sr_type_e_SR_STRING_T =>  {
            let string_val = unsafe { CStr::from_ptr(value.data.string_val) };
            format!("= {}", string_val.to_str().unwrap())
        }
        sr_type_e_SR_BOOL_T => {
            let bool_val = unsafe { value.data.bool_val as bool };
            format!("= {}", bool_val)
        }
        sr_type_e_SR_DECIMAL64_T => {
            let dec64_val = unsafe { value.data.decimal64_val as f64 };
            format!("= {}", dec64_val)
        }
        sr_type_e_SR_INT8_T => {
            let int8_val = unsafe { value.data.int8_val as i8 };
            format!("= {}", int8_val)
        }
        sr_type_e_SR_INT16_T => {
            let int16_val = unsafe { value.data.int16_val as i16 };
            format!("= {}", int16_val)
        }
        sr_type_e_SR_INT32_T => {
            let int32_val = unsafe { value.data.int32_val as i32 };
            format!("= {}", int32_val)
        }
        sr_type_e_SR_INT64_T => {
            let int64_val = unsafe { value.data.int64_val as i64 };
            format!("= {}", int64_val)
        }
        sr_type_e_SR_UINT8_T => {
            let uint8_val = unsafe { value.data.uint8_val as u8 };
            format!("= {}", uint8_val)
        }
        sr_type_e_SR_UINT16_T => {
            let uint16_val = unsafe { value.data.uint16_val as u16 };
            format!("= {}", uint16_val)
        }
        sr_type_e_SR_UINT32_T => {
            let uint32_val = unsafe { value.data.uint32_val as u32 };
            format!("= {}", uint32_val)
        }
        sr_type_e_SR_UINT64_T => {
            let uint64_val = unsafe { value.data.uint64_val as u64 };
            format!("= {}", uint64_val)
        }
        sr_type_e_SR_IDENTITYREF_T => {
            let identityref_val = unsafe { CStr::from_ptr(value.data.identityref_val) };
            format!("= {}", identityref_val.to_str().unwrap())
        }
        sr_type_e_SR_INSTANCEID_T => {
            let instanceid_val = unsafe { CStr::from_ptr(value.data.instanceid_val) };
            format!("= {}", instanceid_val.to_str().unwrap())
        }
        sr_type_e_SR_BITS_T => {
            let bits_val = unsafe { CStr::from_ptr(value.data.bits_val) };
            format!("= {}", bits_val.to_str().unwrap())
        }
        sr_type_e_SR_BINARY_T => {
            let binary_val = unsafe { CStr::from_ptr(value.data.binary_val) };
            format!("= {}", binary_val.to_str().unwrap())
        }
        sr_type_e_SR_ENUM_T => {
            let enum_val = unsafe { CStr::from_ptr(value.data.enum_val) };
            format!("= {}", enum_val.to_str().unwrap())
        }
        sr_type_e_SR_LEAF_EMPTY_T => String::from("(empty leaf)"),
        _ => String::from("(unprintable)"),
    };

    match value.type_ {
        sr_type_e_SR_UNKNOWN_T |
        sr_type_e_SR_CONTAINER_T |
        sr_type_e_SR_CONTAINER_PRESENCE_T |
        sr_type_e_SR_LIST_T |
        sr_type_e_SR_LEAF_EMPTY_T => println!("{}", v),
        _ => println!("{}{}", v, if value.dflt { " [default]" } else { "" }),
    }
}

static SIGTSTP_ONCE: sync::Once = sync::Once::new();
static SIGINT_CAUGHT: sync::atomic::AtomicUsize = sync::atomic::AtomicUsize::new(0);

extern fn sigint_handler(_: i32) {
    SIGINT_CAUGHT.fetch_add(1, sync::atomic::Ordering::SeqCst);
}

pub fn is_sigint_caught() -> bool {
    SIGINT_CAUGHT.load(sync::atomic::Ordering::SeqCst) > 0
}

pub fn signal_init() {
    SIGTSTP_ONCE.call_once(|| unsafe {
        let sa = signal::SigAction::new(
            signal::SigHandler::Handler(sigint_handler),
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        );
        let _ = signal::sigaction(signal::SIGINT, &sa);
    });
}

