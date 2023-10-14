use std::process::Command;

fn main() {
    let output = Command::new(env!("CARGO")).args(["build", "--package", "build-rs-test"]).output().unwrap().stdout;

    println!("{}", std::str::from_utf8(&output).unwrap());

    println!("cargo:rerun-if-changed=../build-rs-test");
    println!("cargo:rerun-if-changed=build.rs");
}
