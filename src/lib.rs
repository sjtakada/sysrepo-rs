#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::mem::zeroed;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::time::Duration;

/// Error.
#[derive(Copy, Clone)]
pub enum SrError {
    Ok = sr_error_e_SR_ERR_OK as isize,
    InvalArg = sr_error_e_SR_ERR_INVAL_ARG as isize,
    Ly = sr_error_e_SR_ERR_LY as isize,
    Sys = sr_error_e_SR_ERR_SYS as isize,
    Nomem = sr_error_e_SR_ERR_NOMEM as isize,
    NotFound = sr_error_e_SR_ERR_NOT_FOUND as isize,
    Exists = sr_error_e_SR_ERR_EXISTS as isize,
    Internal = sr_error_e_SR_ERR_INTERNAL as isize,
    Unsupported = sr_error_e_SR_ERR_UNSUPPORTED as isize,
    ValidationFailed = sr_error_e_SR_ERR_VALIDATION_FAILED as isize,
    OperationFailed = sr_error_e_SR_ERR_OPERATION_FAILED as isize,
    Unauthorized = sr_error_e_SR_ERR_UNAUTHORIZED as isize,
    Locked = sr_error_e_SR_ERR_LOCKED as isize,
    TimeOut = sr_error_e_SR_ERR_TIME_OUT as isize,
    CallbackFailed = sr_error_e_SR_ERR_CALLBACK_FAILED as isize,
    CallbackShelve = sr_error_e_SR_ERR_CALLBACK_SHELVE as isize,
}

/// Log level.
pub enum SrLogLevel {
    None = sr_log_level_t_SR_LL_NONE as isize,
    Error = sr_log_level_t_SR_LL_ERR as isize,
    Warn = sr_log_level_t_SR_LL_WRN as isize,
    Info = sr_log_level_t_SR_LL_INF as isize,
    Debug = sr_log_level_t_SR_LL_DBG as isize,
}

/// Conn Flag.
pub enum SrConnFlag {
    Default = sr_conn_flag_e_SR_CONN_DEFAULT as isize,
    CacheRunning = sr_conn_flag_e_SR_CONN_CACHE_RUNNING as isize,
    NoSchedChanges = sr_conn_flag_e_SR_CONN_NO_SCHED_CHANGES as isize,
    OnSchedFail = sr_conn_flag_e_SR_CONN_ERR_ON_SCHED_FAIL as isize,
}

/// Datastore.
pub enum SrDatastore {
    Startup = sr_datastore_e_SR_DS_STARTUP as isize,
    Running = sr_datastore_e_SR_DS_RUNNING as isize,
    Candidate = sr_datastore_e_SR_DS_CANDIDATE as isize,
    Operational = sr_datastore_e_SR_DS_OPERATIONAL as isize,
}

/// Sysrepo Type.
pub enum SrType {
    Unknown = sr_type_e_SR_UNKNOWN_T as isize,
    List = sr_type_e_SR_LIST_T as isize,
    Container = sr_type_e_SR_CONTAINER_T as isize,
    ContainerPresence = sr_type_e_SR_CONTAINER_PRESENCE_T as isize,
    LeafEmpty = sr_type_e_SR_LEAF_EMPTY_T as isize,
    Notification = sr_type_e_SR_NOTIFICATION_T as isize,
    Binary = sr_type_e_SR_BINARY_T as isize,
    Bits = sr_type_e_SR_BITS_T as isize,
    Bool = sr_type_e_SR_BOOL_T as isize,
    Decimal64 = sr_type_e_SR_DECIMAL64_T as isize,
    Enum = sr_type_e_SR_ENUM_T as isize,
    IdentityRef = sr_type_e_SR_IDENTITYREF_T as isize,
    InstanceId = sr_type_e_SR_INSTANCEID_T as isize,
    Int8 = sr_type_e_SR_INT8_T as isize,
    Int16 = sr_type_e_SR_INT16_T as isize,
    Int32 = sr_type_e_SR_INT32_T as isize,
    Int64 = sr_type_e_SR_INT64_T as isize,
    String = sr_type_e_SR_STRING_T as isize,
    UInt8 = sr_type_e_SR_UINT8_T as isize,
    UInt16 = sr_type_e_SR_UINT16_T as isize,
    UInt32 = sr_type_e_SR_UINT32_T as isize,
    UInt64 = sr_type_e_SR_UINT64_T as isize,
    AnyXml = sr_type_e_SR_ANYXML_T as isize,
    AnyData = sr_type_e_SR_ANYDATA_T as isize,
}

/// Get Oper Flag.
pub enum SrGetOperFlag {
    Default = sr_get_oper_flag_e_SR_OPER_DEFAULT as isize,
    NoState = sr_get_oper_flag_e_SR_OPER_NO_STATE as isize,
    NoConfig = sr_get_oper_flag_e_SR_OPER_NO_CONFIG as isize,
    NoSubs = sr_get_oper_flag_e_SR_OPER_NO_SUBS as isize,
    NoStored = sr_get_oper_flag_e_SR_OPER_NO_STORED as isize,
    WithOrigin = sr_get_oper_flag_e_SR_OPER_WITH_ORIGIN as isize,
}

/// Edit Flag.
pub enum SrEditFlag {
    Default = sr_edit_flag_e_SR_EDIT_DEFAULT as isize,
    NonRecursive = sr_edit_flag_e_SR_EDIT_NON_RECURSIVE as isize,
    Strict = sr_edit_flag_e_SR_EDIT_STRICT as isize,
    Isolate = sr_edit_flag_e_SR_EDIT_ISOLATE as isize,
}

/// Move Position.
pub enum SrMovePosition {
    Before = sr_move_position_e_SR_MOVE_BEFORE as isize,
    After = sr_move_position_e_SR_MOVE_AFTER as isize,
    First = sr_move_position_e_SR_MOVE_FIRST as isize,
    Last = sr_move_position_e_SR_MOVE_LAST as isize,
}

/// Subscribe Flag.
pub enum SrSubcribeFlag {
    Default = sr_subscr_flag_e_SR_SUBSCR_DEFAULT as isize,
    CtxReuse = sr_subscr_flag_e_SR_SUBSCR_CTX_REUSE as isize,
    NoThread = sr_subscr_flag_e_SR_SUBSCR_NO_THREAD as isize,
    Passive = sr_subscr_flag_e_SR_SUBSCR_PASSIVE as isize,
    DoneOnly = sr_subscr_flag_e_SR_SUBSCR_DONE_ONLY as isize,
    Enabled = sr_subscr_flag_e_SR_SUBSCR_ENABLED as isize,
    Update = sr_subscr_flag_e_SR_SUBSCR_UPDATE as isize,
    Unlocked = sr_subscr_flag_e_SR_SUBSCR_UNLOCKED as isize,
    OperMerge = sr_subscr_flag_e_SR_SUBSCR_OPER_MERGE as isize,
}

/// Event.
pub enum SrEvent {
    Update = sr_event_e_SR_EV_UPDATE as isize,
    Change = sr_event_e_SR_EV_CHANGE as isize,
    Done = sr_event_e_SR_EV_DONE as isize,
    Abort = sr_event_e_SR_EV_ABORT as isize,
    Enabled = sr_event_e_SR_EV_ENABLED as isize,
    Rpc = sr_event_e_SR_EV_RPC as isize,
}

/// Change Oper.
pub enum SrChangeOper {
    Created = sr_change_oper_e_SR_OP_CREATED as isize,
    Modified = sr_change_oper_e_SR_OP_MODIFIED as isize,
    Deleted = sr_change_oper_e_SR_OP_DELETED as isize,
    Moved = sr_change_oper_e_SR_OP_MOVED as isize,
}

/// Notification Type.
pub enum SrNotifType {
    Relative = sr_ev_notif_type_e_SR_EV_NOTIF_REALTIME as isize,
    Replay = sr_ev_notif_type_e_SR_EV_NOTIF_REPLAY as isize,
    ReplayComplete = sr_ev_notif_type_e_SR_EV_NOTIF_REPLAY_COMPLETE as isize,
    Stop = sr_ev_notif_type_e_SR_EV_NOTIF_STOP as isize,
}

/// Sysrepo.
pub struct Sysrepo {

    /// Raw Pointer to Connection.
    conn: *mut sr_conn_ctx_t,
}

impl Sysrepo {

    /// Constructor.
    pub fn new(opts: sr_conn_options_t) -> Result<Sysrepo, i32> {
        let mut conn = std::ptr::null_mut();

        let rc = unsafe {
            sr_connect(opts, &mut conn)
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(Sysrepo {
                conn: conn,
            })
        }
    }

    /// Disconnect.
    pub fn disconnect(&mut self) {
        unsafe {
            sr_disconnect(self.conn);
        }
        self.conn = std::ptr::null_mut();
    }

    /// Start session.
    pub fn start_session(&mut self, ds: SrDatastore) -> Result<SysrepoSession, i32> {
        let mut sess = std::ptr::null_mut();
        let rc = unsafe {
            sr_session_start(self.conn, ds as u32, &mut sess)
        };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(SysrepoSession {
                sess: sess,
            })
        }
    }

    /// Set Log Stderr.
    pub fn log_stderr(log_level: SrLogLevel) {
        unsafe {
            sr_log_stderr(log_level as u32);
        }
    }

    /// Set Log Syslog.
    pub fn log_syslog(app_name: &str, log_level: SrLogLevel) {
        let app_name = &app_name[..] as *const _ as *const i8;
        unsafe {
            sr_log_syslog(app_name, log_level as u32);
        }
    }
}

impl Drop for Sysrepo {
    fn drop (&mut self) {
        self.disconnect();
    }
}

/// Sysrepo session.
pub struct SysrepoSession {

    /// Raw Pointer to session.
    sess: *mut sr_session_ctx_t,
}


impl SysrepoSession {

    pub fn new() -> Self {
        Self {
            sess: std::ptr::null_mut(),
        }
    }

    pub fn set_item_str(&mut self, path: &str, value: &str, origin: Option<&str>,
                        opts: u32) -> Result<(), i32> {
        let path = &path[..] as *const _ as *const i8;
        let value = &value[..] as *const _ as *const i8;
        let origin = match origin {
            Some(orig) => &orig[..] as *const _ as *const i8,
            None => std::ptr::null(),
        };

        let rc = unsafe { sr_set_item_str(self.sess, path, value, origin, opts) };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(())
        }
    }

    pub fn apply_changes(&mut self, timeout: Option<Duration>, wait: bool) -> Result<(), i32> {
        let timeout_ms = match timeout {
            Some(timeout) => timeout.as_millis() as u32,
            None => 0,
        };

        let rc = unsafe {
            sr_apply_changes(self.sess, timeout_ms, if wait { 1 } else { 0 })
        };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(())
        }
    }

    pub fn event_notif_subscribe<F>(&mut self, mod_name: &str, xpath: Option<String>,
                                    start_time: Option<time_t>, stop_time: Option<time_t>,
                                    callback: F, _private_data: *mut c_void,
                                    opts: sr_subscr_options_t)
                                    -> Result<SysrepoSubscription, i32>
    where F: FnMut(*mut sr_session_ctx_t, sr_ev_notif_type_t, *const c_char,
                   *const sr_val_t, size_t, time_t) + 'static,
    {
        let mod_name = &mod_name[..] as *const _ as *const i8;
        let xpath = match xpath {
            Some(xpath) => &xpath[..] as *const _ as * const i8,
            None => std::ptr::null_mut(),
        };
        let start_time = match start_time {
            Some(start_time) => start_time,
            None => 0,
        };
        let stop_time = match stop_time {
            Some(stop_time) => stop_time,
            None => 0,
        };

        let mut subscr: *mut sr_subscription_ctx_t = unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
        let data = Box::into_raw(Box::new(callback));
        let rc = unsafe {
            sr_event_notif_subscribe(self.sess, mod_name, xpath, start_time, stop_time,
                                     Some(SysrepoSession::call_event_notif::<F>),
                                     data as *mut _, opts, &mut subscr)
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let mut ss = SysrepoSubscription::new();
            ss.subscr = subscr;
            Ok(ss)
        }
    }

    unsafe extern "C" fn call_event_notif<F>(
        sess: *mut sr_session_ctx_t,
        notif_type: sr_ev_notif_type_t,
        path: *const c_char,
        values: *const sr_val_t,
        values_cnt: size_t,
        timestamp: time_t,
        private_data: *mut c_void)
    where F: FnMut(*mut sr_session_ctx_t, sr_ev_notif_type_t, *const c_char,
                   *const sr_val_t, size_t, time_t),
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        callback(sess, notif_type, path, values, values_cnt, timestamp);
    }
}

impl Drop for SysrepoSession {
    fn drop (&mut self) {
        unsafe {
            sr_session_stop(self.sess);
        }
    }
}

/// Sysrepo Subscription.
pub struct SysrepoSubscription {

    /// Raw Pointer to subscription.
    subscr: *mut sr_subscription_ctx_t,
}

impl SysrepoSubscription {

    pub fn new() -> Self {
        Self {
            subscr: std::ptr::null_mut(),
        }
    }
}

/*
impl Drop for SysrepoSubscription {
    fn drop (&mut self) {
        unsafe {
            
        }
    }
}
*/
