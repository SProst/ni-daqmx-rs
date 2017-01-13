extern crate bindgen;

use std::io::prelude::*;
use std::fs::File;

fn main(){
    /*
    let mut bindings = bindgen::Builder::new("C:\\Program Files (x86)\\National Instruments\\Shared\\ExternalCompilerSupport\\C\\include\\NIDAQmx.h");
    bindings.link("C:\\Program Files (x86)\\National Instruments\\Shared\\ExternalCompilerSupport\\C\\lib64\\msvc\\NIDAQmx.lib", bindgen::LinkType::Static);
    // Generate the bindings to a string so we can wrap them
    // instead of going through the `write_to_file` API.
    let generated_bindings = bindings.generate().expect("Failed to generate bindings");
    // Now open the file we'll write the generated bindings too
    let mut file = File::create("ni_daq_mx_gen.rs").expect("Failed to open file");
    // Wrap the bindings in a `pub mod` before writing bindgen's output
    file.write(format!("pub mod {} {{\n", "ni_daq_mx_gen").as_bytes()).unwrap();
    file.write(generated_bindings.to_string().as_bytes()).unwrap();
    file.write(b"}").unwrap();
    */
}
