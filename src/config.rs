// sxd - config.rs
// ---------------
// made by matissoss
// licensed under MPL 2.0

use std::path::PathBuf;

use crate::booltable::BoolTable8;

pub const DIFF_FLAG: u8 = 0x0;
pub const COLOR_FLAG: u8 = 0x1;
pub const HELP_FLAG: u8 = 0x2;
pub const VERSION_FLAG: u8 = 0x3;
pub const CHAR_FLAG: u8 = 0x4;

#[derive(Debug)]
pub struct Config {
    paths: [PathBuf; 2],
    line_width: u8,
    flags: BoolTable8,
}

impl Config {
    pub fn from_args(args: Vec<String>) -> Self {
        parse_args(args)
    }
    pub fn new() -> Self {
        Self {
            paths: [PathBuf::new(), PathBuf::new()],
            flags: BoolTable8::new(),
            line_width: 16,
        }
    }
    pub fn line_width(&self) -> u8 {
        self.line_width
    }
    fn set_path(&mut self, idx: usize, path: PathBuf) -> bool {
        if idx < 2 {
            self.paths[idx] = path;
            true
        } else {
            false
        }
    }
    pub fn path_1(&self) -> Option<&PathBuf> {
        if self.paths[0] != PathBuf::new() {
            Some(&self.paths[0])
        } else {
            None
        }
    }
    pub fn path_2(&self) -> Option<&PathBuf> {
        if self.get_flag(DIFF_FLAG) {
            if self.paths[1] != PathBuf::new() {
                Some(&self.paths[1])
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_flag(&self, idx: u8) -> bool {
        self.flags.get(idx).unwrap_or(false)
    }
    fn set_flag(&mut self, idx: u8) {
        self.flags.set(idx, true)
    }
}

fn parse_args(args: Vec<String>) -> Config {
    let mut config = Config::new();

    let mut pidx = 0;
    for arg in &args[1..] {
        if let Some((key, val)) = arg.split_once('=') {
            match key {
                "-1" => {
                    let _ = config.set_path(0, PathBuf::from(val));
                }
                "-2" => {
                    config.set_flag(DIFF_FLAG);
                    let _ = config.set_path(1, PathBuf::from(val));
                }
                "-lw" => {
                    config.line_width = val.parse().unwrap_or(16);
                }
                _ => {}
            }
        } else {
            match arg.as_str() {
                "--diff" => config.set_flag(DIFF_FLAG),
                "-v" | "--version" => config.set_flag(VERSION_FLAG),
                "-h" | "--help" => config.set_flag(HELP_FLAG),
                "-c" => config.set_flag(COLOR_FLAG),
                "-C" => config.set_flag(CHAR_FLAG),
                _ => {
                    if !config.set_path(pidx, arg.into()) {
                        panic!("you provided too many files!");
                    }
                    pidx += 1;
                }
            }
        }
    }

    config
}
