// sxd - color.rs
// --------------
// made by matissoss
// licensed under MPL 2.0

pub fn redc(c: char) {
    print!("\x1b[0;31m{}\x1b[0m", c)
}
pub fn grnc(c: char) {
    print!("\x1b[0;32m{}\x1b[0m", c)
}
pub fn yelc(c: char) {
    print!("\x1b[0;33m{}\x1b[0m", c)
}
pub fn bluc(c: char) {
    print!("\x1b[0;34m{}\x1b[0m", c)
}

pub fn red(c: u8) {
    print!("\x1b[0;31m{:02x}\x1b[0m", c)
}
pub fn grn(c: u8) {
    print!("\x1b[0;32m{:02x}\x1b[0m", c)
}
pub fn yel(c: u8) {
    print!("\x1b[0;33m{:02x}\x1b[0m", c)
}
pub fn blu(c: u8) {
    print!("\x1b[0;34m{:02x}\x1b[0m", c)
}

pub fn print_byte(cu: u8, bl: bool) {
    let c = cu as char;
    if !bl {
        print!("{:02x}", cu);
        return;
    }
    if c.is_alphabetic() {
        grn(cu)
    } else if c.is_ascii_punctuation() {
        blu(cu)
    } else if c.is_numeric() {
        yel(cu)
    } else {
        red(cu)
    }
}

pub fn print_char(c: char, bl: bool) {
    if !bl {
        print!("{}", c);
        return;
    }
    if c == '.' {
        redc(c)
    } else if c.is_alphabetic() {
        grnc(c)
    } else if c.is_ascii_punctuation() {
        bluc(c)
    } else if c.is_numeric() {
        yelc(c)
    } else {
        redc(c)
    }
}
