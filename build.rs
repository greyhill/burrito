use std::env;

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

    if !matched {
        panic!("Please set either MATLAB_LIB_PATH or OCTAVE_LIB_PATH");
    }
}

