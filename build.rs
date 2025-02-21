
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // note that there are a number of downsides to this approach, the comments
    // below detail how to improve the portability of these commands.
    Command::new("gcc").args(&["src/deco.c", "-c","-ljpeg","-O3", "-fPIC","-g", "-o"])
                       .arg(&format!("{}/decoc.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libdecoc.a", "decoc.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();
    Command::new("touch").arg("what")
                      .status().unwrap();

    println!("cargo:rerun-if-changed=src/deco.c");
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=decoc");
}
