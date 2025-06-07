// sxd - src/engine.rs
// -------------------
// made by matissoss
// licensed under MPL 2.0

use std::{
    fs,
    path::PathBuf
};

use crate::color;
use crate::config::*;

pub fn hex_dump(conf: Config) {
    if conf.get_flag(DIFF_FLAG) {
        diff(conf);
        return;
    }

    let lines = get_content(conf.path_1(), conf.line_width());

    let mut address : u64 = 0x0;
    let use_color = conf.get_flag(COLOR_FLAG);
    let char_flag = conf.get_flag(CHAR_FLAG);
    let line_width_u64 = conf.line_width() as u64;
    for l in lines {
        print!("{:08x}:", address);
        for b in &l {
            print!(" ");
            crate::color::print_byte(*b, use_color);
        }
        if char_flag {
            print!("{} ; ", " ".repeat(((line_width_u64 as usize) - l.len()) * 3));
            for b in l {
                color::print_char(
                if b.is_ascii_alphanumeric() {
                    b as char
                } else {
                    '.'
                }
                , use_color);
            }
        }
        println!();
        address += line_width_u64;
    }
}

pub fn diff(conf: Config) {
    let lines_1 = get_content(conf.path_1(), conf.line_width());
    let lines_2 = get_content(conf.path_2(), conf.line_width());

    let color_bool = conf.get_flag(COLOR_FLAG);
    let line_width_u64 = conf.line_width() as u64;
    if lines_1.len() != lines_2.len() {
        print!("Files differ in length!");
        if conf.get_flag(LEAVE_ERROR){
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }
    for idx in 0..lines_1.len() {
        if lines_1[idx] != lines_2[idx] {
            let address = idx as u64 * line_width_u64;
            print!("\n{:08x}:", address);
            for b in &lines_1[idx] {
                print!(" ");
                color::print_byte(*b, color_bool);
            }
            print!("\n{}", " ".repeat(10));
            for b in &lines_2[idx] {
                color::print_byte(*b, color_bool);
                print!(" ");
            }
        }
    }
}

fn get_content(path: Option<&PathBuf>, lw: u8) -> Vec<Vec<u8>> {
    if path.is_none() {
        panic!("You tried to do hex dump without giving any path :)");
    }
    let path = path.unwrap();
    if !path.exists(){
        panic!("File you tried to access, doesn't exist!");
    }
    let buf = fs::read(path).expect("Error occured, while trying to read file!");
    
    split_lines(buf, lw)
}

pub fn split_lines(vec: Vec<u8>, line_width: u8) -> Vec<Vec<u8>> {
    let usize_line = line_width as usize;
    let mut vec_new = Vec::new();

    let mut tmp_buf = Vec::with_capacity(usize_line);
    let mut counter = 1;
    for b in vec {
        if counter <= usize_line {
            tmp_buf.push(b);
        } else {
            vec_new.push(tmp_buf);
            tmp_buf = Vec::with_capacity(usize_line);
            counter = 0;
        }
        counter += 1;
    }
    if !tmp_buf.is_empty(){
        vec_new.push(tmp_buf);
    }
    vec_new
}
