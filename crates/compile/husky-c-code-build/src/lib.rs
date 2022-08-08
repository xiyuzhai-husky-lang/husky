use colored::Colorize;
use std::process::Command;

pub fn build_single_file_to_lib(out_dir: &str, filename: &str) {
    let cc_output = Command::new("cc")
        .arg("-c")
        .arg(format!("{out_dir}/{filename}.c"))
        .arg("-o")
        .arg(format!("{out_dir}/{filename}.o"))
        .output()
        .expect("cc failed");
    if cc_output.stderr.len() > 0 {
        panic!("{}", std::str::from_utf8(&cc_output.stderr).unwrap().red())
    }
    let ar_output = Command::new("ar")
        .arg("-rcs")
        .arg(format!("{out_dir}/lib{filename}.a"))
        .arg(format!("{out_dir}/{filename}.o"))
        .output()
        .expect("ar failed");
    if ar_output.stderr.len() > 0 {
        panic!("{}", std::str::from_utf8(&ar_output.stderr).unwrap().red())
    }
}
