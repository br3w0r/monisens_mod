use libc::{c_char, c_void};
use std::ffi::CString;

#[no_mangle]
pub extern "C" fn mod_version() -> u8 {
    1
}

#[repr(C)]
#[derive(Clone)]
pub enum ConnParamType {
    Bool,
    Int,
    Float,
    String,
}

struct LConnParamConf {
    name: CString,
    typ: ConnParamType,
}

#[repr(C)]
pub struct ConnParamConf {
    name: *const c_char,
    typ: ConnParamType,
}

impl From<&LConnParamConf> for ConnParamConf {
    fn from(v: &LConnParamConf) -> Self {
        Self {
            name: v.name.as_ptr(),
            typ: v.typ.clone(),
        }
    }
}

#[repr(C)]
pub struct DeviceConf {
    connection_params: *const ConnParamConf,
    connection_params_len: i32,
}

impl From<&Vec<ConnParamConf>> for DeviceConf {
    fn from(v: &Vec<ConnParamConf>) -> Self {
        Self {
            connection_params: v.as_ptr(),
            connection_params_len: (v.len() as i32),
        }
    }
}

pub struct Module {
    params: Vec<LConnParamConf>,
}

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
        Box::from_raw(ptr);
        self.0 = std::ptr::null::<c_void>() as *mut _;
    }

    pub fn from_module(module: Module) -> Self {
        let reference = Box::leak(Box::new(module));
        Self((reference as *mut Module) as _)
    }
}

type InitFn = unsafe extern "C" fn(*mut Handle);
type ObtainDeviceConfFn = unsafe extern "C" fn(*const Handle, *mut c_void, ObtainDeviceConfCallback);

type ObtainDeviceConfCallback = unsafe extern "C" fn(*mut c_void, *const DeviceConf);

type DestroyFn = unsafe extern "C" fn(*mut Handle);

#[repr(C)]
pub struct Functions {
    init: InitFn,
    obtain_device_conf: ObtainDeviceConfFn,
    destroy: DestroyFn,
}

#[no_mangle]
pub unsafe extern "C" fn functions() -> Functions {
    Functions {
        init,
        obtain_device_conf,
        destroy,
    }
}

#[no_mangle]
pub unsafe extern "C" fn init(sel: *mut Handle) {
    let m = Module {
        params: vec![
            LConnParamConf {
                name: CString::new("IP").unwrap(),
                typ: ConnParamType::String,
            },
            LConnParamConf {
                name: CString::new("Password").unwrap(),
                typ: ConnParamType::Int,
            },
        ],
    };

    *sel = Handle::from_module(m);
}

#[no_mangle]
pub unsafe extern "C" fn obtain_device_conf(
    sel: *const Handle,
    obj: *mut c_void,
    callback: ObtainDeviceConfCallback,
) {
    let module = (*sel).as_module();
    let params_vec: Vec<ConnParamConf> = module.params.iter().map(|x| x.into()).collect();
    let params: DeviceConf = (&params_vec).into();

    callback(obj, &params);
}

#[no_mangle]
pub unsafe extern "C" fn destroy(sel: *mut Handle) {
    (*sel).destroy();
}
