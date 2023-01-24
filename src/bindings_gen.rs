/* automatically generated by rust-bindgen 0.63.0 */

#![allow(warnings)]

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ConnParamType {
    ConnParamBool = 0,
    ConnParamInt = 1,
    ConnParamFloat = 2,
    ConnParamString = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConnParamInfo {
    pub name: *mut ::std::os::raw::c_char,
    pub typ: ConnParamType,
}
#[test]
fn bindgen_test_layout_ConnParamInfo() {
    const UNINIT: ::std::mem::MaybeUninit<ConnParamInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ConnParamInfo>(),
        16usize,
        concat!("Size of: ", stringify!(ConnParamInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<ConnParamInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(ConnParamInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ConnParamInfo),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typ) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ConnParamInfo),
            "::",
            stringify!(typ)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConnectInfo {
    pub connection_params: *mut ConnParamInfo,
    pub connection_params_len: i32,
}
#[test]
fn bindgen_test_layout_DeviceConnectInfo() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConnectInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConnectInfo>(),
        16usize,
        concat!("Size of: ", stringify!(DeviceConnectInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConnectInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConnectInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connection_params) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConnectInfo),
            "::",
            stringify!(connection_params)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connection_params_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConnectInfo),
            "::",
            stringify!(connection_params_len)
        )
    );
}
pub type device_info_callback = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void, arg2: *mut DeviceConnectInfo),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConnParam {
    pub name: *mut ::std::os::raw::c_char,
    pub value: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ConnParam() {
    const UNINIT: ::std::mem::MaybeUninit<ConnParam> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ConnParam>(),
        16usize,
        concat!("Size of: ", stringify!(ConnParam))
    );
    assert_eq!(
        ::std::mem::align_of::<ConnParam>(),
        8usize,
        concat!("Alignment of ", stringify!(ConnParam))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ConnParam),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ConnParam),
            "::",
            stringify!(value)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConnectConf {
    pub connection_params: *mut ConnParam,
    pub connection_params_len: i32,
}
#[test]
fn bindgen_test_layout_DeviceConnectConf() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConnectConf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConnectConf>(),
        16usize,
        concat!("Size of: ", stringify!(DeviceConnectConf))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConnectConf>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConnectConf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connection_params) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConnectConf),
            "::",
            stringify!(connection_params)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connection_params_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConnectConf),
            "::",
            stringify!(connection_params_len)
        )
    );
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SensorDataType {
    SensorDataInt = 0,
    SensorDataFloat = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DeviceConfInfoEntryType {
    DeviceConfInfoEntryTypeSection = 0,
    DeviceConfInfoEntryTypeString = 1,
    DeviceConfInfoEntryTypeInt = 2,
    DeviceConfInfoEntryTypeIntRange = 3,
    DeviceConfInfoEntryTypeFloat = 4,
    DeviceConfInfoEntryTypeFloatRange = 5,
    DeviceConfInfoEntryTypeJSON = 6,
    DeviceConfInfoEntryTypeChoiceList = 7,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryString {
    pub required: bool,
    pub def: *mut ::std::os::raw::c_char,
    pub min_len: *mut i32,
    pub max_len: *mut i32,
    pub match_regex: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryString() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryString> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryString>(),
        40usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryString))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryString>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryString))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryString),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryString),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min_len) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryString),
            "::",
            stringify!(min_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_len) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryString),
            "::",
            stringify!(max_len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).match_regex) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryString),
            "::",
            stringify!(match_regex)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryInt {
    pub required: bool,
    pub def: *mut i32,
    pub lt: *mut i32,
    pub gt: *mut i32,
    pub neq: *mut i32,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryInt() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryInt> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryInt>(),
        40usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryInt))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryInt>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryInt))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryInt),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryInt),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lt) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryInt),
            "::",
            stringify!(lt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gt) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryInt),
            "::",
            stringify!(gt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).neq) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryInt),
            "::",
            stringify!(neq)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryIntRange {
    pub required: bool,
    pub def_from: *mut i32,
    pub def_to: *mut i32,
    pub min: i32,
    pub max: i32,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryIntRange() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryIntRange> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryIntRange>(),
        32usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryIntRange))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryIntRange>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryIntRange))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryIntRange),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def_from) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryIntRange),
            "::",
            stringify!(def_from)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def_to) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryIntRange),
            "::",
            stringify!(def_to)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryIntRange),
            "::",
            stringify!(min)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryIntRange),
            "::",
            stringify!(max)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryFloat {
    pub required: bool,
    pub def: *mut f32,
    pub lt: *mut f32,
    pub gt: *mut f32,
    pub neq: *mut f32,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryFloat() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryFloat> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryFloat>(),
        40usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryFloat))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryFloat>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryFloat))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloat),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloat),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lt) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloat),
            "::",
            stringify!(lt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gt) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloat),
            "::",
            stringify!(gt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).neq) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloat),
            "::",
            stringify!(neq)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryFloatRange {
    pub required: bool,
    pub def_from: *mut f32,
    pub def_to: *mut f32,
    pub min: f32,
    pub max: f32,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryFloatRange() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryFloatRange> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryFloatRange>(),
        32usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryFloatRange))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryFloatRange>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryFloatRange))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloatRange),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def_from) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloatRange),
            "::",
            stringify!(def_from)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def_to) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloatRange),
            "::",
            stringify!(def_to)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).min) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloatRange),
            "::",
            stringify!(min)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryFloatRange),
            "::",
            stringify!(max)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryJSON {
    pub required: bool,
    pub def: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryJSON() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryJSON> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryJSON>(),
        16usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryJSON))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryJSON>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryJSON))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryJSON),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryJSON),
            "::",
            stringify!(def)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntryChoiceList {
    pub required: bool,
    pub def: *mut i32,
    pub choices: *mut *mut ::std::os::raw::c_char,
    pub chioces_len: i32,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntryChoiceList() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntryChoiceList> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntryChoiceList>(),
        32usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntryChoiceList))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntryChoiceList>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntryChoiceList))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).required) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryChoiceList),
            "::",
            stringify!(required)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).def) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryChoiceList),
            "::",
            stringify!(def)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).choices) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryChoiceList),
            "::",
            stringify!(choices)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).chioces_len) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntryChoiceList),
            "::",
            stringify!(chioces_len)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfoEntry {
    pub id: i32,
    pub name: *mut ::std::os::raw::c_char,
    pub typ: DeviceConfInfoEntryType,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_DeviceConfInfoEntry() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfoEntry> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfoEntry>(),
        32usize,
        concat!("Size of: ", stringify!(DeviceConfInfoEntry))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfoEntry>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfoEntry))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntry),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntry),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).typ) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntry),
            "::",
            stringify!(typ)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfoEntry),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfInfo {
    pub device_confs: *mut DeviceConfInfoEntry,
    pub device_confs_len: i32,
}
#[test]
fn bindgen_test_layout_DeviceConfInfo() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfInfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfInfo>(),
        16usize,
        concat!("Size of: ", stringify!(DeviceConfInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfInfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).device_confs) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfo),
            "::",
            stringify!(device_confs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).device_confs_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfInfo),
            "::",
            stringify!(device_confs_len)
        )
    );
}
pub type device_conf_info_callback = ::std::option::Option<
    unsafe extern "C" fn(obj: *mut ::std::os::raw::c_void, info: *mut DeviceConfInfo),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConfEntry {
    pub id: i32,
    pub data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_DeviceConfEntry() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConfEntry> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConfEntry>(),
        16usize,
        concat!("Size of: ", stringify!(DeviceConfEntry))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConfEntry>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConfEntry))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfEntry),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConfEntry),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DeviceConf {
    pub confs: *mut DeviceConfEntry,
    pub confs_len: i32,
}
#[test]
fn bindgen_test_layout_DeviceConf() {
    const UNINIT: ::std::mem::MaybeUninit<DeviceConf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<DeviceConf>(),
        16usize,
        concat!("Size of: ", stringify!(DeviceConf))
    );
    assert_eq!(
        ::std::mem::align_of::<DeviceConf>(),
        8usize,
        concat!("Alignment of ", stringify!(DeviceConf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).confs) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConf),
            "::",
            stringify!(confs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).confs_len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(DeviceConf),
            "::",
            stringify!(confs_len)
        )
    );
}
pub type mod_version_fn = ::std::option::Option<unsafe extern "C" fn() -> u8>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Functions {
    pub init:
        ::std::option::Option<unsafe extern "C" fn(handler: *mut *mut ::std::os::raw::c_void)>,
    pub obtain_device_info: ::std::option::Option<
        unsafe extern "C" fn(
            handler: *mut ::std::os::raw::c_void,
            obj: *mut ::std::os::raw::c_void,
            callback: device_info_callback,
        ),
    >,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(handler: *mut ::std::os::raw::c_void)>,
    pub connect_device: ::std::option::Option<
        unsafe extern "C" fn(
            handler: *mut ::std::os::raw::c_void,
            connect_info: *mut DeviceConnectConf,
        ) -> u8,
    >,
    pub obtain_device_conf_info: ::std::option::Option<
        unsafe extern "C" fn(
            handler: *mut ::std::os::raw::c_void,
            obj: *mut ::std::os::raw::c_void,
            callback: device_conf_info_callback,
        ),
    >,
    pub configure_device: ::std::option::Option<
        unsafe extern "C" fn(handler: *mut ::std::os::raw::c_void, conf: *mut DeviceConf) -> u8,
    >,
}
#[test]
fn bindgen_test_layout_Functions() {
    const UNINIT: ::std::mem::MaybeUninit<Functions> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<Functions>(),
        48usize,
        concat!("Size of: ", stringify!(Functions))
    );
    assert_eq!(
        ::std::mem::align_of::<Functions>(),
        8usize,
        concat!("Alignment of ", stringify!(Functions))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).init) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Functions),
            "::",
            stringify!(init)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obtain_device_info) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Functions),
            "::",
            stringify!(obtain_device_info)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).destroy) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Functions),
            "::",
            stringify!(destroy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connect_device) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(Functions),
            "::",
            stringify!(connect_device)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).obtain_device_conf_info) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(Functions),
            "::",
            stringify!(obtain_device_conf_info)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).configure_device) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(Functions),
            "::",
            stringify!(configure_device)
        )
    );
}
pub type functions_fn = ::std::option::Option<unsafe extern "C" fn() -> Functions>;
