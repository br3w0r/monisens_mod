mod bindings_gen;
mod c_parser;

use bindings_gen::{self as bg, DeviceConnectInfo};

use libc::c_void;
use std::thread::{self, JoinHandle};
use std::{
    ffi::{c_char, CStr, CString},
    ptr::null,
    time::{Duration, SystemTime},
};

use async_macros::select;
use async_std::{
    channel::{self, Receiver, Sender},
    io::{self, WriteExt},
    net,
    task::{self},
};
use lazy_static::lazy_static;
use lipsum::lipsum_words_with_rng;
use rand::thread_rng;
use regex::Regex;
use urlencoding::encode;

const CONN_PARAM_IP: &str = "IP";
const CONN_PARAM_PORT: &str = "Port";
const CONN_PARAM_MESSAGE: &str = "Message";

const DEV_CONF_ID_DEV_COMM_INTERVAL: i32 = 1;
const DEV_CONF_ID_MSG_TYPE: i32 = 2;
const DEV_CONF_ID_MSG_TEXT: i32 = 3;

const DEV_CONF_MSG_TYPE_IDX_TIMESTAMP: i32 = 0;
const DEV_CONF_MSG_TYPE_IDX_CPU: i32 = 1;
const DEV_CONF_MSG_TYPE_IDX_TEXT: i32 = 2;

lazy_static! {
    static ref RE_IP: Regex = Regex::new(r"^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$").unwrap();
}

#[no_mangle]
pub extern "C" fn mod_version() -> u8 {
    1
}

struct ConnParamInfo {
    name: CString,
    typ: bg::ConnParamType,
}

impl From<&ConnParamInfo> for bg::ConnParamInfo {
    fn from(v: &ConnParamInfo) -> Self {
        Self {
            name: v.name.as_ptr() as _,
            typ: v.typ.clone(),
            info: null::<c_void>() as _,
        }
    }
}

impl From<&Vec<bg::ConnParamInfo>> for bg::DeviceConnectInfo {
    fn from(v: &Vec<bg::ConnParamInfo>) -> Self {
        Self {
            connection_params: v.as_ptr() as _,
            connection_params_len: (v.len() as i32),
        }
    }
}

pub struct Module {
    params: Vec<ConnParamInfo>,
    conn_conf: Option<ConnConf>,
    device_conf: Option<DeviceConf>,

    // Process flow
    thread_handle: Option<JoinHandle<()>>,
    stop_tx: Option<Sender<()>>,
}

#[repr(transparent)]
pub struct Handle(*mut c_void);

impl Handle {
    /// # Panics
    /// Panics if `self.0` == null.
    pub unsafe fn as_module(&self) -> &'static mut Module {
        let ptr = self.0 as *mut Module;
        ptr.as_mut().unwrap() // Expect null checks before
    }

    /// # Safety
    /// `self.0` != null.
    pub unsafe fn destroy(&mut self) {
        let ptr = self.0 as *mut Module;
        let _ = Box::from_raw(ptr);
        self.0 = std::ptr::null::<c_void>() as *mut _;
    }

    pub fn from_module(module: Module) -> Self {
        let reference = Box::leak(Box::new(module));
        Self((reference as *mut Module) as _)
    }
}

#[no_mangle]
pub unsafe extern "C" fn functions() -> bg::Functions {
    bg::Functions {
        init: Some(init),
        obtain_device_info: Some(obtain_device_info),
        destroy: Some(destroy),
        connect_device: Some(connect_device),
        obtain_device_conf_info: Some(obtain_device_conf_info),
        configure_device: Some(configure_device),
        obtain_sensor_type_infos: Some(obtain_sensor_type_infos),
        start: Some(start),
        stop: Some(stop),
    }
}

#[no_mangle]
pub unsafe extern "C" fn init(sel: *mut *mut c_void) {
    let m = Module {
        params: vec![
            ConnParamInfo {
                name: CString::new(CONN_PARAM_IP).unwrap(),
                typ: bg::ConnParamType::ConnParamString,
            },
            ConnParamInfo {
                name: CString::new(CONN_PARAM_PORT).unwrap(),
                typ: bg::ConnParamType::ConnParamInt,
            },
            ConnParamInfo {
                name: CString::new(CONN_PARAM_MESSAGE).unwrap(),
                typ: bg::ConnParamType::ConnParamString,
            },
        ],
        conn_conf: None,
        device_conf: Some(DeviceConf {
            comm_interval: 1,
            msg: MessageType::Timestamp,
        }),
        thread_handle: None,
        stop_tx: None,
    };

    *sel = Handle::from_module(m).0;
}

#[no_mangle]
pub unsafe extern "C" fn obtain_device_info(
    handler: *mut c_void,
    obj: *mut c_void,
    callback: bg::device_info_callback,
) {
    let module = Handle(handler).as_module();
    let params_vec: Vec<bg::ConnParamInfo> = module.params.iter().map(|x| x.into()).collect();
    let mut params: DeviceConnectInfo = (&params_vec).into();

    callback.unwrap()(obj, &mut params as _);
}

#[no_mangle]
pub unsafe extern "C" fn destroy(sel: *mut c_void) {
    stop(sel);
    Handle(sel).destroy();
}

const DEVICE_ERROR_NONE: u8 = 0;

#[repr(u8)]
pub enum DeviceErr {
    DeviceErrConn = 1,
    DeviceErrParams = 2,
}

#[no_mangle]
pub extern "C" fn connect_device(handler: *mut c_void, confs: *mut bg::DeviceConnectConf) -> u8 {
    if let Err(err) = connect_device_impl(handler, confs) {
        err as _
    } else {
        DEVICE_ERROR_NONE
    }
}

fn connect_device_impl(
    handler: *mut c_void,
    confs: *mut bg::DeviceConnectConf,
) -> Result<(), DeviceErr> {
    let conf = ConnConf::new(confs)?;

    if let Err(_) = task::block_on(send_message_async(
        conf.ip.clone(),
        conf.port,
        conf.message.clone(),
    )) {
        return Err(DeviceErr::DeviceErrConn);
    }

    let module = unsafe { Handle(handler).as_module() };

    module.conn_conf = Some(conf);

    Ok(())
}

extern "C" fn obtain_device_conf_info(
    _: *mut c_void,
    obj: *mut c_void,
    callback: bg::device_conf_info_callback,
) {
    let mut entries = Vec::with_capacity(2);

    // ENTRY: Device comminucation interval
    let entry_interval_name = CString::new("Device comminucation interval (in seconds)").unwrap();
    let mut entry_interval_lt = 300i32;
    let mut entry_interval_gt = 0i32;
    let mut entry_interval = bg::DeviceConfInfoEntryInt {
        required: true,
        def: null::<i32>() as _,
        lt: &mut entry_interval_lt as _,
        gt: &mut entry_interval_gt as _,
        neq: null::<i32>() as _,
    };
    entries.push(bg::DeviceConfInfoEntry {
        id: DEV_CONF_ID_DEV_COMM_INTERVAL,
        name: entry_interval_name.as_ptr() as _,
        typ: bg::DeviceConfInfoEntryType::DeviceConfInfoEntryTypeInt,
        data: &mut entry_interval as *mut bg::DeviceConfInfoEntryInt as *mut c_void,
    });

    // ENTRY: Message
    // SUB ENTRY: Message > Message type
    let entry_msg_type_name = CString::new("Message type").unwrap();
    let entry_msg_type_timestamp: CString = CString::new("UNIX Timestamp").unwrap();
    let entry_msg_type_cpu_msg = CString::new("CPU Usage").unwrap();
    let entry_msg_type_txt_msg: CString = CString::new("Text Message").unwrap();

    let entry_msg_type_type_list = vec![
        entry_msg_type_timestamp.as_ptr(), // const DEV_CONF_MSG_TYPE_IDX_TIMESTAMP
        entry_msg_type_cpu_msg.as_ptr(),   // const DEV_CONF_MSG_TYPE_IDX_CPU
        entry_msg_type_txt_msg.as_ptr(),   // const DEV_CONF_MSG_TYPE_IDX_TEXT
    ];

    let entry_msg_type_default = 0;
    let mut entry_msg_type = bg::DeviceConfInfoEntryChoiceList {
        required: true,
        def: &entry_msg_type_default as *const i32 as *mut i32,
        choices: entry_msg_type_type_list.as_ptr() as _,
        chioces_len: entry_msg_type_type_list.len() as _,
    };

    // SUB ENTRY: Message > Message text
    let entry_msg_text_name = CString::new("Message text (if type text)").unwrap();
    let entry_msg_default = CString::new("TEST").unwrap();
    let mut entry_msg_max_len = 255i32;
    let mut entry_msg = bg::DeviceConfInfoEntryString {
        required: false,
        def: entry_msg_default.as_ptr() as _,
        min_len: null::<i32>() as _,
        max_len: &mut entry_msg_max_len as _,
        match_regex: null::<i8>() as _,
    };

    let entry_msg_section = vec![
        bg::DeviceConfInfoEntry {
            id: DEV_CONF_ID_MSG_TYPE,
            name: entry_msg_type_name.as_ptr() as _,
            typ: bg::DeviceConfInfoEntryType::DeviceConfInfoEntryTypeChoiceList,
            data: &mut entry_msg_type as *mut bg::DeviceConfInfoEntryChoiceList as *mut c_void,
        },
        bg::DeviceConfInfoEntry {
            id: DEV_CONF_ID_MSG_TEXT,
            name: entry_msg_text_name.as_ptr() as _,
            typ: bg::DeviceConfInfoEntryType::DeviceConfInfoEntryTypeString,
            data: &mut entry_msg as *mut bg::DeviceConfInfoEntryString as *mut c_void,
        },
    ];

    let entry_msg_section_name = CString::new("Message").unwrap();
    let mut entry_msg_section = bg::DeviceConfInfo {
        device_confs: entry_msg_section.as_ptr() as _,
        device_confs_len: entry_msg_section.len() as _,
    };

    entries.push(bg::DeviceConfInfoEntry {
        id: 0, // No need of id for entry of type 'Section'
        name: entry_msg_section_name.as_ptr() as _,
        typ: bg::DeviceConfInfoEntryType::DeviceConfInfoEntryTypeSection,
        data: &mut entry_msg_section as *mut bg::DeviceConfInfo as *mut c_void,
    });

    let mut conf_info = bg::DeviceConfInfo {
        device_confs: entries.as_ptr() as _,
        device_confs_len: entries.len() as _,
    };

    unsafe { callback.unwrap()(obj, &mut conf_info as _) };
}

#[derive(Default, Debug)]
struct ConnConf {
    ip: String,
    port: u16,
    message: String,
}

impl ConnConf {
    fn new(confs_raw: *mut bg::DeviceConnectConf) -> Result<Self, DeviceErr> {
        if confs_raw.is_null() {
            return Err(DeviceErr::DeviceErrParams);
        }

        let confs = unsafe {
            std::slice::from_raw_parts(
                (*confs_raw).connection_params,
                (*confs_raw).connection_params_len as usize,
            )
        };

        let mut res_conf = ConnConf::default();

        for conf in confs {
            if let Ok(name) = unsafe { CStr::from_ptr(conf.name) }.to_str() {
                match name {
                    CONN_PARAM_IP => {
                        if let Some(ip) = c_parser::as_string(conf.value) {
                            res_conf.ip = ip;
                        } else {
                            return Err(DeviceErr::DeviceErrParams);
                        }
                    }
                    CONN_PARAM_PORT => {
                        if let Some(port) = c_parser::as_from_str::<u16>(conf.value) {
                            res_conf.port = port;
                        } else {
                            return Err(DeviceErr::DeviceErrParams);
                        }
                    }
                    CONN_PARAM_MESSAGE => {
                        if let Some(msg) = c_parser::as_string(conf.value) {
                            res_conf.message = msg;
                        } else {
                            return Err(DeviceErr::DeviceErrParams);
                        }
                    }
                    _ => {
                        return Err(DeviceErr::DeviceErrParams);
                    }
                }
            } else {
                return Err(DeviceErr::DeviceErrParams);
            }
        }

        if !res_conf.validate() {
            return Err(DeviceErr::DeviceErrParams);
        }

        Ok(res_conf)
    }

    fn validate(&self) -> bool {
        if !RE_IP.is_match(&self.ip) {
            return false;
        }

        if self.port <= 0 {
            return false;
        }

        if self.message.len() == 0 {
            return false;
        }

        true
    }
}

#[derive(Debug)]
enum MessageType {
    Timestamp,
    CPU,
    Text(String),
}

#[derive(Debug)]
struct DeviceConf {
    comm_interval: i32,
    msg: MessageType,
}

impl DeviceConf {
    pub fn new(raw: *mut bg::DeviceConf) -> Result<DeviceConf, DeviceErr> {
        let raw_slice = unsafe { std::slice::from_raw_parts((*raw).confs, (*raw).confs_len as _) };

        let mut res_conf = DeviceConf::default();
        let mut txt = String::new();

        for conf in raw_slice {
            match conf.id {
                DEV_CONF_ID_DEV_COMM_INTERVAL => {
                    res_conf.comm_interval = unsafe { *(conf.data as *const i32) };
                }
                DEV_CONF_ID_MSG_TYPE => match unsafe { *(conf.data as *const i32) } {
                    DEV_CONF_MSG_TYPE_IDX_TIMESTAMP => {
                        res_conf.msg = MessageType::Timestamp;
                    }
                    DEV_CONF_MSG_TYPE_IDX_CPU => {
                        res_conf.msg = MessageType::CPU;
                    }
                    DEV_CONF_MSG_TYPE_IDX_TEXT => {
                        res_conf.msg = MessageType::Text(String::new());
                    }
                    _ => {
                        return Err(DeviceErr::DeviceErrParams);
                    }
                },
                DEV_CONF_ID_MSG_TEXT => {
                    let text = conf.data as *const c_char;

                    if !text.is_null() {
                        txt = c_parser::str_from_c_char(text);
                    }
                }
                _ => {
                    return Err(DeviceErr::DeviceErrParams);
                }
            }
        }

        if let MessageType::Text(_) = res_conf.msg {
            if txt.is_empty() {
                return Err(DeviceErr::DeviceErrParams);
            }

            res_conf.msg = MessageType::Text(txt);
        }

        Ok(res_conf)
    }
}

impl Default for DeviceConf {
    fn default() -> Self {
        Self {
            comm_interval: Default::default(),
            msg: MessageType::Timestamp,
        }
    }
}

extern "C" fn configure_device(handler: *mut c_void, conf: *mut bg::DeviceConf) -> u8 {
    if let Err(err) = configure_device_impl(handler, conf) {
        err as _
    } else {
        DEVICE_ERROR_NONE
    }
}

fn configure_device_impl(handler: *mut c_void, conf: *mut bg::DeviceConf) -> Result<(), DeviceErr> {
    let device_conf = DeviceConf::new(conf)?;

    let module = unsafe { Handle(handler).as_module() };

    module.device_conf = Some(device_conf);

    println!("{:?}", module.device_conf);

    Ok(())
}

extern "C" fn obtain_sensor_type_infos(
    _: *mut c_void,
    obj: *mut c_void,
    callback: bg::sensor_type_infos_callback,
) -> u8 {
    // SENSOR: Test Server
    let test_server_sensor_type_info_name = CString::new("test_server").unwrap();
    let test_server_type_info_response_name = CString::new("response").unwrap();
    let test_server_type_info_response = bg::SensorDataTypeInfo {
        name: test_server_type_info_response_name.as_ptr() as _,
        typ: bg::SensorDataType::SensorDataTypeString,
    };

    let test_server_type_info_timestamp_name = CString::new("timestamp").unwrap();
    let test_server_type_info_timestamp = bg::SensorDataTypeInfo {
        name: test_server_type_info_timestamp_name.as_ptr() as _,
        typ: bg::SensorDataType::SensorDataTypeTimestamp,
    };

    let test_server_sensor_type_info_vec = vec![
        test_server_type_info_response,
        test_server_type_info_timestamp,
    ];

    let test_server_sensor_type_info = bg::SensorTypeInfo {
        name: test_server_sensor_type_info_name.as_ptr() as _,
        data_type_infos_len: test_server_sensor_type_info_vec.len() as _,
        data_type_infos: test_server_sensor_type_info_vec.as_ptr() as _,
    };

    // SENSOR: Test Sensor
    let test_sensor_sensor_type_info_name = CString::new("test_sensor").unwrap();
    let test_sensor_sensor_type_info_data_name = CString::new("data").unwrap();
    let test_sensor_sensor_type_info_data = bg::SensorDataTypeInfo {
        name: test_sensor_sensor_type_info_data_name.as_ptr() as _,
        typ: bg::SensorDataType::SensorDataTypeString,
    };
    let test_sensor_sensor_type_info_timestamp_name = CString::new("timestamp").unwrap();
    let test_sensor_sensor_type_info_timestamp = bg::SensorDataTypeInfo {
        name: test_sensor_sensor_type_info_timestamp_name.as_ptr() as _,
        typ: bg::SensorDataType::SensorDataTypeTimestamp,
    };

    let test_sensor_sensor_type_info_vec = vec![
        test_sensor_sensor_type_info_data,
        test_sensor_sensor_type_info_timestamp,
    ];

    let test_sensor_sensor_type_info = bg::SensorTypeInfo {
        name: test_sensor_sensor_type_info_name.as_ptr() as _,
        data_type_infos_len: test_sensor_sensor_type_info_vec.len() as _,
        data_type_infos: test_sensor_sensor_type_info_vec.as_ptr() as _,
    };

    // Sensor infos
    let sensor_type_infos_vec = vec![test_server_sensor_type_info, test_sensor_sensor_type_info];

    let sensor_type_infos = bg::SensorTypeInfos {
        sensor_type_infos_len: sensor_type_infos_vec.len() as _,
        sensor_type_infos: sensor_type_infos_vec.as_ptr() as _,
    };

    unsafe { callback.unwrap()(obj, &sensor_type_infos as *const _ as *mut _) };

    DEVICE_ERROR_NONE
}

struct MsgHandle(*mut c_void);

unsafe impl Send for MsgHandle {}

extern "C" fn start(
    handler: *mut c_void,
    msg_handler: *mut c_void,
    handle_func: bg::handle_msg_func,
) -> u8 {
    let module = unsafe { Handle(handler).as_module() };

    let handle = MsgHandle(msg_handler);
    let sleep_duration =
        Duration::from_secs(module.device_conf.as_ref().unwrap().comm_interval as _);
    let (tx, rx) = channel::bounded(1);

    let t = thread::spawn(move || {
        // Copy msg handle
        let handle = handle;
        let sleep_duration = sleep_duration;
        let stop_rx = rx;
        loop {
            let sensor_name = CString::new("test_sensor").unwrap();
            let sensor_data_name = CString::new("data").unwrap();
            let sensor_data = CString::new(lipsum_words_with_rng(thread_rng(), 4)).unwrap();

            let sensor_timestamp_name = CString::new("timestamp").unwrap();
            let sensor_timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;
            let data = vec![
                bg::SensorMsgData {
                    name: sensor_data_name.as_ptr() as _,
                    typ: bg::SensorDataType::SensorDataTypeString,
                    data: sensor_data.as_ptr() as _,
                },
                bg::SensorMsgData {
                    name: sensor_timestamp_name.as_ptr() as _,
                    typ: bg::SensorDataType::SensorDataTypeTimestamp,
                    data: &sensor_timestamp as *const i64 as *mut _,
                },
            ];

            let msg = bg::SensorMsg {
                name: sensor_name.as_ptr() as _,
                data: data.as_ptr() as *mut _,
                data_len: data.len() as _,
            };

            let msg_data = bg::Message {
                typ: bg::MessageType::MessageTypeSensor,
                data: &msg as *const bg::SensorMsg as *mut _,
            };

            unsafe { handle_func.unwrap()(handle.0, msg_data) };

            if !task::block_on(timer_with_cancel(sleep_duration, &stop_rx)) {
                break;
            }
        }
    });

    module.thread_handle = Some(t);
    module.stop_tx = Some(tx);

    DEVICE_ERROR_NONE
}

/// Returns `true` if timeout has passed and no message was received from `stop_rx`
async fn timer_with_cancel(dur: Duration, stop_rx: &Receiver<()>) -> bool {
    // TODO: use atomic bool, create another future for reading the channel and write true to the atomic bool if timeout is passed
    let sleep_fut = async {
        task::sleep(dur).await;

        true
    };
    let stop_fut = async {
        let _ = stop_rx.recv().await;

        false
    };

    select!(stop_fut, sleep_fut).await
}

extern "C" fn stop(handler: *mut ::std::os::raw::c_void) -> u8 {
    let module = unsafe { Handle(handler).as_module() };

    let opt_handle = std::mem::replace(&mut module.thread_handle, None);
    if let Some(handle) = opt_handle {
        task::spawn(async {
            module.stop_tx.as_ref().unwrap().close();
        });

        handle.join().unwrap();
    }

    DEVICE_ERROR_NONE
}

// --------------------------- Module-specific functions ---------------------------

async fn send_message_async(host: String, port: u16, msg: String) -> io::Result<()> {
    let addr = host + ":" + port.to_string().as_str();
    let mut stream = net::TcpStream::connect(addr).await?;
    let req = format!(
        "GET /?msg={} HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
        encode(&msg)
    );
    stream.write_all(req.as_bytes()).await?;

    // let mut stdout = io::stdout();
    // let mut buf = vec![0; 1024];
    // stream.read(&mut buf).await?;
    // println!("{}", String::from_utf8(buf).unwrap());
    Ok(())
}
