use std;
use std::ffi::{CString, CStr};
use libc::{c_char, c_void};

pub fn daq_mx_create_task(task_name: &str) -> i32{
    println!("{}", task_name);
    let name = CString::new("").unwrap();
    let mut task_handle = 0;
    let status = unsafe{
        DAQmxCreateTask(name.as_ptr(), &mut task_handle)
    };
    status
}

#[link(name = "C:\\Program Files (x86)\\National Instruments\\Shared\\ExternalCompilerSupport\\C\\lib64\\msvc\\NIDAQmx")]
extern "stdcall"{
    fn DAQmxLoadTask(task_name: *const c_char, task_handle: *mut u32) -> i32;
    fn DAQmxCreateTask(task_name: *const c_char, task_handle: *mut u32) -> i32;
    fn DAQmxStartTask(task_handle: c_void) -> i32;
    fn DAQmxStopTask(task_handle: c_void) -> i32;
    fn DAQmxClearTask(task_handle: c_void) -> i32;
}
