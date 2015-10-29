use std::env;
use std::path::{Path, PathBuf};
use std::fs::metadata;
use std::process::Command;
use std::io::Read;

fn get_path() -> Option<String> {
    for (key, val) in env::vars() {
        if key == "PATH" {
            return Some(val)
        }
    }
    None
}

fn main() {
    let mut matched = false;

    for (key, val) in env::vars() {
        match (&key[..], &val[..], matched) {
            ("MATLAB_LIB_PATH", _, true) |
                ("OCTAVE_LIB_PATH", _, true) => {
                    panic!("Please specify only MATLAB_LIB_PATH or OCTAVE_LIB_PATH");
            },
            ("MATLAB_LIB_PATH", path, false) => {
                println!("cargo:rustc-link-search={}", path);
                println!("cargo:rustc-cfg=matlab");
                matched = true;
            },
            ("OCTAVE_LIB_PATH", path, false) => {
                println!("cargo:rustc-link-search={}", path);
                println!("cargo:rustc-cfg=octave");
                matched = true;
            },
            _ => { }
        };
    }

    if let (Some(path), false) = (get_path(), matched) {
        for path_item in path.split(":") {
            // look for octave-config
            let mut oct_path = PathBuf::from(path_item);
            oct_path.push("octave-config");
            if let Ok(_) = metadata(&oct_path) {
                if let Ok(output) = Command::new(&oct_path).arg("--print").arg("OCTLIBDIR").output() {
                    if let Ok(txt) = String::from_utf8(output.stdout) {
                        println!("cargo:rustc-link-search={}", txt);
                        println!("cargo:rustc-cfg=octave");
                        matched = true;
                    }
                }
            }

            // look for matlab-config
            // TODO
        }
    }

    if !matched {
        panic!("Please set either MATLAB_LIB_PATH or OCTAVE_LIB_PATH");
    }
}

