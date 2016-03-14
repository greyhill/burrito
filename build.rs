use std::env;
use std::path::PathBuf;
use std::fs::metadata;
use std::process::Command;

fn get_path() -> Option<String> {
    for (key, val) in env::vars() {
        if key == "PATH" {
            return Some(val)
        }
    }
    None
}

fn get_matlab_path(ml_output: &str) -> Option<&str> {
    for line in ml_output.split("\n") {
        let mut it = line.split("=");
        match (it.next(), it.next()) {
            (Some("LD_LIBRARY_PATH"), Some(p)) => {
                return Some(p)
            },
            _ => {
            },
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

            if matched  {
                break;
            }

            // look for matlab
            let mut matlab_path = PathBuf::from(path_item);
            matlab_path.push("matlab");
            if let Ok(_) = metadata(&matlab_path) {
                if let Ok(output) = Command::new(&matlab_path).arg("-e").output() {
                    if let Ok(txt) = String::from_utf8(output.stdout) {
                        if let Some(p) = get_matlab_path(&txt) {
                            println!("cargo:rustc-link-search={}", p);
                            println!("cargo:rustc-cfg=matlab");
                            matched = true;
                        }
                    }
                }
            }
        }
    }

    if !matched {
        panic!("Please set either MATLAB_LIB_PATH or OCTAVE_LIB_PATH");
    }
}

