extern crate libc;
mod ni_daq_mx;
mod ni_daq_mx_gen;

fn main() {
    let result = ni_daq_mx::daq_mx_create_task("Test");
    println!("{:?}", result);
}
