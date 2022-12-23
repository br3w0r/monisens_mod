mod bindings_gen;

use bindings_gen::{self as bg, DeviceConnectInfo};

use libc::{c_char, c_void};
use std::{borrow::BorrowMut, ffi::CString};

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
                name: CString::new("IP").unwrap(),
                typ: bg::ConnParamType::ConnParamString,
            },
            ConnParamInfo {
                name: CString::new("Password").unwrap(),
                typ: bg::ConnParamType::ConnParamInt,
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

#[no_mangle]
pub unsafe extern "C" fn connect_device(
    handler: *mut c_void,
    connect_info: *mut bg::DeviceConnectConf,
) -> u8 {
    todo!()
}
