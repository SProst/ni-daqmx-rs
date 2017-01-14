use std;
use std::ffi::{CString, CStr};
use ni_daq_mx_gen::ni_daq_mx_gen;

pub fn daq_mx_create_task(task_name: &str) -> std::os::raw::c_void{
    let name = CString::new("").unwrap();
    let mut task_handle = 0 as *mut std::os::raw::c_void ;
    let status = unsafe{
        ni_daq_mx_gen::DAQmxCreateTask(name.as_ptr() as *mut i8 , &mut task_handle )
    };
    println!("{:?}", task_handle);
    task_handle as std::os::raw::c_void;
}
