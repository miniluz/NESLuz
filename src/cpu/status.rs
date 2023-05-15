#[derive(Debug)]
pub enum Flags {
    Carry,
    Zero,
    InterruptDisable,
    Decimal,
    Overflow,
    Negative,
}

impl From<Flags> for u8 {
    fn from(value: Flags) -> Self {
        use Flags::*;
        match value {
            Carry => 0b0000_0001,
            Zero => 0b0000_0010,
            InterruptDisable => 0b0000_0100,
            Decimal => 0b0000_1000,
            Overflow => 0b0100_0000,
            Negative => 0b1000_0000,
        }
    }
}

#[derive(Debug)]
pub struct Status {
    flags: u8,
}

impl Status {
    pub fn new() -> Status {
        Status { flags: 0b0000_0000 }
    }

    pub fn set(&mut self, flag: Flags, condition: bool) {
        if condition {
            self.flags |= u8::from(flag);
        } else {
            self.flags &= 0b1111_1111 ^ u8::from(flag);
        }
    }

    pub fn get(&self, flag: Flags) -> bool {
        self.flags & u8::from(flag) != 0b0000_0000
    }
}
