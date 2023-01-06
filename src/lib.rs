mod bindings_gen;

use async_std::{
    io::{self, WriteExt},
    net, task,
};
use lazy_static::lazy_static;
use regex::Regex;

use bindings_gen::{self as bg, DeviceConnectInfo};

use libc::c_void;
use std::ffi::{CStr, CString};

const CONN_PARAM_IP: &str = "IP";
const CONN_PARAM_PORT: &str = "Port";
const CONN_PARAM_MESSAGE: &str = "Message";

lazy_static! {
    static ref RE_IP: Regex = Regex::new(r"(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])").unwrap();
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
    };

    *sel = Handle::from_module(m).0;
}

#[no_mangle]
pub unsafe extern "C" fn obtain_device_info(
    sel: *mut c_void,
    obj: *mut c_void,
    callback: bg::device_info_callback,
) {
    let module = Handle(sel).as_module();
    let params_vec: Vec<bg::ConnParamInfo> = module.params.iter().map(|x| x.into()).collect();
    let mut params: DeviceConnectInfo = (&params_vec).into();

    callback.unwrap()(obj, &mut params as _);
}

#[no_mangle]
pub unsafe extern "C" fn destroy(sel: *mut c_void) {
    Handle(sel).destroy();
}

#[repr(u8)]
pub enum ConnDeviceErr {
    ConnDeviceErrorNone = 0,
    ConnDeviceErrorConn = 1,
    ConnDeviceErrorParams = 2,
}

#[no_mangle]
pub extern "C" fn connect_device(handler: *mut c_void, confs: *mut bg::DeviceConnectConf) -> u8 {
    if let Err(err) = connect_device_impl(handler, confs) {
        err as _
    } else {
        ConnDeviceErr::ConnDeviceErrorNone as _
    }
}

fn connect_device_impl(
    handler: *mut c_void,
    confs: *mut bg::DeviceConnectConf,
) -> Result<(), ConnDeviceErr> {
    let conf = ConnConf::new(confs)?;

    // TODO
    println!("{:?}", conf);

    if let Err(err) = task::block_on(send_message_async(conf.ip, conf.port, conf.message)) {
        return Err(ConnDeviceErr::ConnDeviceErrorConn);
    }

    Ok(())
}

#[derive(Default, Debug)]
struct ConnConf {
    ip: String,
    port: u16,
    message: String,
}

impl ConnConf {
    fn new(confs_raw: *mut bg::DeviceConnectConf) -> Result<Self, ConnDeviceErr> {
        if confs_raw.is_null() {
            return Err(ConnDeviceErr::ConnDeviceErrorParams);
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
                            return Err(ConnDeviceErr::ConnDeviceErrorParams);
                        }
                    }
                    CONN_PARAM_PORT => {
                        if let Some(port) = c_parser::as_from_str::<u16>(conf.value) {
                            res_conf.port = port;
                        } else {
                            return Err(ConnDeviceErr::ConnDeviceErrorParams);
                        }
                    }
                    CONN_PARAM_MESSAGE => {
                        if let Some(msg) = c_parser::as_string(conf.value) {
                            res_conf.message = msg;
                        } else {
                            return Err(ConnDeviceErr::ConnDeviceErrorParams);
                        }
                    }
                    _ => {
                        return Err(ConnDeviceErr::ConnDeviceErrorParams);
                    }
                }
            } else {
                return Err(ConnDeviceErr::ConnDeviceErrorParams);
            }
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

mod c_parser {
    use std::{
        ffi::{c_char, CStr},
        str::FromStr,
    };

    pub fn str_from_c_char(raw: *mut c_char) -> String {
        let cstr = unsafe { CStr::from_ptr(raw) };

        String::from_utf8_lossy(cstr.to_bytes()).to_string()
    }

    pub fn as_string(raw: *mut c_char) -> Option<String> {
        Some(str_from_c_char(raw))
    }

    pub fn as_from_str<F: FromStr>(raw: *mut c_char) -> Option<F> {
        let s = str_from_c_char(raw);

        let res = s.parse::<F>();
        if let Ok(val) = res {
            Some(val)
        } else {
            None
        }
    }
}

async fn send_message_async(host: String, port: u16, msg: String) -> io::Result<()> {
    let addr = host + ":" + port.to_string().as_str();
    let mut stream = net::TcpStream::connect(addr).await?;
    let req = format!(
        "GET /?msg={} HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n",
        msg
    );
    stream.write_all(req.as_bytes()).await?;

    // let mut stdout = io::stdout();
    // let mut buf = vec![0; 1024];
    // stream.read(&mut buf).await?;
    // println!("{}", String::from_utf8(buf).unwrap());
    Ok(())
}
