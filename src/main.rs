// sxd - main.rs
// -------------
// made by matissoss
// licensed under MPL 2.0

use std::{
    env,
    process
};

pub mod engine;
pub mod booltable;
pub mod config;
pub mod color;

fn main() {
    std::panic::set_hook(Box::new(panic_handler));
    let conf = config::Config::from_args(env::args().collect());
    if conf.get_flag(config::HELP_FLAG) {
        help();
        process::exit(0)
    }
    if conf.get_flag(config::VERSION_FLAG) {
        version();
        process::exit(0);
    }
    engine::hex_dump(conf);
}

fn panic_handler(panic: &std::panic::PanicHookInfo){
    let ctt : &str = if let Some(str) = panic.payload().downcast_ref::<&str>() {
        str
    } else if let Some(str) = panic.payload().downcast_ref::<String>() {
        str
    } else {
        ""
    };
    eprintln!("error: {ctt}");
    std::process::exit(1);
}

fn help(){
    println!(
"sxd - simple hex dump v25-06-alpha1
---------------------
made by matissoss
licensed under MPL 2.0
---
USAGE:
$BIN [FLAG(s)]
---
[FLAG(s)]:
- -1=[PATH] ; file 1
- -2=[PATH] ; file 2 - used with --diff
- --diff    ; compares two files
- -C        ; print characters beside hex dump (n)
- -c        ; prints characters in color using ANSI escape codes
- -h        ; prints this message
- -v        ; prints version
- -lw=[VAL] ; specifies line width used in hex dump, default = 16"
)
}
fn version(){
    println!(
"sxd - 25-06-alpha1"
)
}
