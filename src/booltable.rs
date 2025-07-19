// rasmx86_64 - src/shr/booltable.rs
// ---------------------------------
// made by matissoss
// licensed under MPL 2.0

#[repr(transparent)]
#[derive(Debug)]
pub struct BoolTable8 {
    data: u8,
}

impl BoolTable8 {
    pub fn new() -> Self {
        Self { data: 0 }
    }
    pub fn set(&mut self, idx: u8, bool: bool) {
        let mask = 0xFF ^ (0b1 << idx);
        self.data = (self.data & mask) | ((bool as u8) << idx)
    }
    pub fn get(&self, idx: u8) -> Option<bool> {
        if idx < 8 {
            let tmp = 0x01 << idx;
            Some(self.data & tmp == tmp)
        } else {
            None
        }
    }
}
