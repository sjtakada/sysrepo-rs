#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::slice;
use std::mem;
use std::mem::zeroed;
use std::mem::size_of;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::time::Duration;
use std::ffi::CStr;
use std::collections::HashMap;

use libc;

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

/// Lyd Anydata Value Type.
#[derive(Clone, Copy)]
pub enum LydAnyDataValueType {
    ConstString = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_CONSTSTRING as isize,
    String = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_STRING as isize,
    Json = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_JSON as isize,
    JsonD = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_JSOND as isize,
    Sxml = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_SXML as isize,
    Sxmld = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_SXMLD as isize,
    Xml = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_XML as isize,
    Datatree = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_DATATREE as isize,
    Lyb = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_LYB as isize,
    Lybd = LYD_ANYDATA_VALUETYPE_LYD_ANYDATA_LYBD as isize,
}

/// Sysrepo Value Slice.
pub struct SysrepoValues {

    /// Pointer to raw sr_val_t array.
    values: *mut sr_val_t,

    /// Number of values.
    len: u64,

    /// Owned flag.
    owned: bool,
}

impl SysrepoValues {

    pub fn new() -> Self {
        Self {
            values: std::ptr::null_mut(),
            len: 0,
            owned: false,
        }
    }

    pub fn from(values: *mut sr_val_t, len: u64, owned: bool) -> Self {
        Self {
            values: values,
            len: len,
            owned: owned,
        }
    }

    pub fn as_slice(&mut self) -> &[sr_val_t] {
        unsafe {
            slice::from_raw_parts(self.values, self.len as usize)
        }
    }

    pub fn set_owned(&mut self) {
        self.owned = true;
    }
}

impl Drop for SysrepoValues {

    fn drop (&mut self) {
        if self.owned {
            unsafe {
                sr_free_values(self.values, self.len);
            }
        }
    }
}

/// Sysrepo.
pub struct Sysrepo {

    /// Raw Pointer to Connection.
    conn: *mut sr_conn_ctx_t,

    /// Map from sid.sr to SysrepoSession.
    sessions: HashMap<u32, SysrepoSession>,
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
                sessions: HashMap::new(),
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

    /// Add session to map.
    pub fn insert_session(&mut self, id: u32, sess: SysrepoSession) {
        self.sessions.insert(id, sess);
    }

    /// Add session to map.
    pub fn remove_session(&mut self, id: u32) {
        self.sessions.remove(&id);
    }

    /// Lookup session from map.
    pub fn lookup_session(&mut self, id: &u32) -> Option<&mut SysrepoSession> {
        self.sessions.get_mut(id)
    }

    /// Start session.
    pub fn start_session(&mut self, ds: SrDatastore) -> Result<&mut SysrepoSession, i32> {
        let mut sess = std::ptr::null_mut();
        let rc = unsafe {
            sr_session_start(self.conn, ds as u32, &mut sess)
        };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = unsafe {
                sr_session_get_id(sess)
            };
            self.insert_session(id, SysrepoSession::from(sess));
            Ok(self.sessions.get_mut(&id).unwrap())
        }
    }

    // Get context.
    pub fn get_context(&mut self) -> LibYangCtx {
        LibYangCtx::from(unsafe { sr_get_context(self.conn) })
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
        self.sessions.drain();
        self.disconnect();
    }
}

/// Sysrepo session.
pub struct SysrepoSession {

    /// Raw Pointer to session.
    sess: *mut sr_session_ctx_t,

    /// Incremental subscription ID.
    id: usize,

    /// Map from raw pointer to subscription.
    subscrs: HashMap<usize, SysrepoSubscription>,
}


impl SysrepoSession {

    pub fn new() -> Self {
        Self {
            sess: std::ptr::null_mut(),
            id: 0,
            subscrs: HashMap::new(),
        }
    }

    pub fn from(sess: *mut sr_session_ctx_t) -> Self {
        Self {
            sess: sess,
            id: 0,
            subscrs: HashMap::new(),
        }
    }

    pub fn get_id(&self) -> u32 {
        unsafe {
            sr_session_get_id(self.sess)
        }
    }

    pub fn insert_subscription(&mut self, subscr: SysrepoSubscription) -> usize {
        self.id += 1;
        self.subscrs.insert(self.id, subscr);
        self.id
    }

    pub fn remove_subscription(&mut self, id: usize) {
        self.subscrs.remove(&id);
    }

    pub fn lookup_subscription(&mut self, id: &usize) -> Option<&mut SysrepoSubscription> {
        self.subscrs.get_mut(&id)
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
        let timeout_ms = timeout.map_or(0, |timeout| timeout.as_millis() as u32);

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
                                    -> Result<&mut SysrepoSubscription, i32>
    where F: FnMut(u32, sr_ev_notif_type_t, &str,
                   &[sr_val_t], time_t) + 'static,
    {
        let mod_name = &mod_name[..] as *const _ as *const i8;
        let xpath = xpath.map_or(std::ptr::null_mut(),
                                 |xpath| &xpath[..] as *const _ as *mut i8);
        let start_time = start_time.unwrap_or(0);
        let stop_time = stop_time.unwrap_or(0);

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
            let id = self.insert_subscription(SysrepoSubscription::from(subscr));
            Ok(self.subscrs.get_mut(&id).unwrap())
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
    where F: FnMut(u32, sr_ev_notif_type_t,
                   &str, &[sr_val_t], time_t),
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        let path: &CStr = CStr::from_ptr(path);
        let vals: &[sr_val_t] = slice::from_raw_parts(values, values_cnt as usize);

        callback(sr_session_get_id(sess), notif_type, path.to_str().unwrap(), vals, timestamp);
    }

    pub fn rpc_subscribe<F>(&mut self, xpath: Option<String>,
                            callback: F, _private_data: *mut c_void,
                            priority: u32, opts: sr_subscr_options_t)
                            -> Result<&mut SysrepoSubscription, i32>
    where F: FnMut(u32, &str, &[sr_val_t],
                   sr_event_t, u32) -> Vec<sr_val_t> + 'static,
    {
        let mut subscr: *mut sr_subscription_ctx_t = unsafe { zeroed::<*mut sr_subscription_ctx_t>() };
        let data = Box::into_raw(Box::new(callback));

        let rc = unsafe {
            match xpath {
                Some(xpath) => {
                    let xpath = &xpath[..] as *const _ as *mut i8;
                    sr_rpc_subscribe(self.sess, xpath, Some(SysrepoSession::call_rpc::<F>),
                                     data as *mut _, priority, opts, &mut subscr)
                }
                None => {
                    sr_rpc_subscribe(self.sess, std::ptr::null_mut(), Some(SysrepoSession::call_rpc::<F>),
                                     data as *mut _, priority, opts, &mut subscr)
                }
            }
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            let id = self.insert_subscription(SysrepoSubscription::from(subscr));
            Ok(self.subscrs.get_mut(&id).unwrap())
        }
    }

    unsafe extern "C" fn call_rpc<F>(
        sess: *mut sr_session_ctx_t,
        op_path: *const c_char,
        input: *const sr_val_t,
        input_cnt: size_t,
        event: sr_event_t,
        request_id: u32,
        output: *mut *mut sr_val_t,
        output_cnt: *mut u64,
        private_data: *mut c_void) -> i32
    where F: FnMut(u32, &str, &[sr_val_t],
                   sr_event_t, u32) -> Vec<sr_val_t>,
    {
        let callback_ptr = private_data as *mut F;
        let callback = &mut *callback_ptr;

        let op_path: &CStr = CStr::from_ptr(op_path);
        let inputs: &[sr_val_t] = slice::from_raw_parts(input, input_cnt as usize);

        let vec = callback(sr_session_get_id(sess), op_path.to_str().unwrap(), inputs, event, request_id);
        *output = libc::malloc(mem::size_of::<sr_val_t>() * vec.len()) as *mut sr_val_t;
        let ptr: *mut *mut sr_val_t = output;

        for i in 0..vec.len() {
            unsafe {
                let val = vec[i];
                std::ptr::copy(&val, (*ptr).offset(i as isize), 1);
            }
        }
        *output_cnt = vec.len() as u64;

        sr_error_e_SR_ERR_OK as i32
    }

    pub fn event_notif_send_tree(&mut self, notif: &LydNode) -> Result<(), i32> {
        let rc = unsafe {
            sr_event_notif_send_tree(self.sess, notif.get_node())
        };
        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(())
        }
    }

    pub fn rpc_send(&mut self, path: &str, input: Option<Vec<sr_val_t>>,
                    timeout: Option<Duration>) -> Result<SysrepoValues, i32> {
        let path = &path[..] as *const _ as *mut i8;
        let (input, input_cnt) = match input {
            Some(mut input) => (input.as_mut_ptr(), input.len() as u64),
            None => (std::ptr::null_mut(), 0)
        };
        let timeout = timeout.map_or(0, |timeout| timeout.as_millis() as u32);

        let mut output: *mut sr_val_t = unsafe { zeroed::<*mut sr_val_t>() };
        let mut output_count: u64 = 0;

        let rc = unsafe {
            sr_rpc_send(self.sess, path, input, input_cnt, timeout,
                        &mut output, &mut output_count)
        };

        if rc != SrError::Ok as i32 {
            Err(rc)
        } else {
            Ok(SysrepoValues::from(output, output_count, true))
        }
    }
}

impl Drop for SysrepoSession {
    fn drop (&mut self) {
        self.subscrs.drain();

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

    pub fn from(subscr: *mut sr_subscription_ctx_t) -> Self {
        Self {
            subscr: subscr,
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

/// Lib Yang Context.
///  It just holds raw pointer, but does not own the object.
pub struct LibYangCtx {

    /// Raw Pointer to Lib Yang Context.
    ly_ctx: *const ly_ctx,
}

impl LibYangCtx {

    /// Constructo from raw pointer.
    pub fn from(ly_ctx: *const ly_ctx) -> Self {
        Self {
            ly_ctx: ly_ctx,
        }
    }

    pub fn get_ctx(&self) -> *const ly_ctx {
        self.ly_ctx
    }
}

/// LibYang data node.
pub struct LydNode {

    /// Raw pointer to LibYang data node.
    node: *mut lyd_node,
}

impl LydNode {

    pub fn from(node: *mut lyd_node) -> Self {
        Self {
            node: node,
        }
    }

    pub fn get_node(&self) -> *mut lyd_node {
        self.node
    }

    pub fn free_withsiblings(&self) {
        unsafe {
            lyd_free_withsiblings(self.node);
        }
    }
}

/// LibYang data value.
pub struct LydValue {

    value_type: LydAnyDataValueType,

    /// TBD: It is string for now.
    ///      It has to be variable length of byte array.
    value: String,
}

impl LydValue {

    pub fn from_string(s: String) -> Self {
        Self {
            value_type: LydAnyDataValueType::ConstString,
            value: s.clone(),
        }
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn get_value_raw(&self) -> *mut c_void {
        self.get_value() as *const _ as *mut c_void
    }

    pub fn get_type(&self) -> LydAnyDataValueType {
        self.value_type
    }
}

/// Lib Yang Utilities.
pub struct LibYang {

}

impl LibYang {

    pub fn lyd_new_path(node: Option<&LydNode>, ly_ctx: Option<&LibYangCtx>,
                        path: &str, value: Option<&LydValue>, options: i32) -> Option<LydNode> {

        let node = node.map_or(std::ptr::null_mut(), |node| node.get_node());
        let ctx = ly_ctx.map_or(std::ptr::null_mut(), |ly_ctx| ly_ctx.get_ctx() as *mut ly_ctx);
        let path = &path[..] as *const _ as * const i8;
        let node = match value {
            Some(value) => {
                let value_type = value.get_type();
                let value = String::from(value.get_value());
                let value = &value[..] as *const _ as *mut c_void;

                unsafe {
                    lyd_new_path(node, ctx, path, value, value_type as u32, options)
                }
            }
            None => {
                unsafe {
                    lyd_new_path(node, ctx, path, std::ptr::null_mut(),
                                 LydAnyDataValueType::ConstString as u32, options)
                }
            }
        };
        // Value type fallbacks to ConstString, is it OK?

        if node != std::ptr::null_mut() {
            Some(LydNode::from(node))
        } else {
            None
        }
    }
}


