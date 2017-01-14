mod ni_daq_mx;
mod ni_daq_mx_gen;

fn main() {
    let task_handle = ni_daq_mx::daq_mx_create_task("");
}
