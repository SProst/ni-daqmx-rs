extern crate libc;
mod ni_daq_mx;

fn main() {
    let result = ni_daq_mx::daq_mx_create_task("Test");
    println!("{:?}", result);
}
