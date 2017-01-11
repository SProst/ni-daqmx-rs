use std;
use std::ffi::{CString, CStr};
use libc::{c_char, c_void};

#[link(name = "C:\\Program Files (x86)\\National Instruments\\NI-DAQ\\DAQmx ANSI C Dev\\lib\\msvc\\NIDAQmx")]
extern "stdcall"{
    fn DAQmxLoadTask(task_name: *const c_char, task_handle: *mut c_void) -> i32;
    fn DAQmxCreateTask(task_name: *const c_char, task_handle: *mut c_void) -> i32;
}
