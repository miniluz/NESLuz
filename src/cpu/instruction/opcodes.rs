pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6d;
pub const ADC_ABSOLUTE_X: u8 = 0x7d;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZERO_PAGE: u8 = 0x25;
pub const AND_ZERO_PAGE_X: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2d;
pub const AND_ABSOLUTE_X: u8 = 0x3d;
pub const AND_ABSOLUTE_Y: u8 = 0x39;
pub const AND_INDIRECT_X: u8 = 0x21;
pub const AND_INDIRECT_Y: u8 = 0x31;

pub const LDA_IMMEDIATE: u8 = 0xa9;
pub const LDA_ZERO_PAGE: u8 = 0xa5;
pub const LDA_ZERO_PAGE_X: u8 = 0xb5;
pub const LDA_ABSOLUTE: u8 = 0xad;
pub const LDA_ABSOLUTE_X: u8 = 0xbd;
pub const LDA_ABSOLUTE_Y: u8 = 0xb9;
pub const LDA_INDIRECT_X: u8 = 0xa1;
pub const LDA_INDIRECT_Y: u8 = 0xb1;

pub const LDX_IMMEDIATE: u8 = 0xa2;
pub const LDX_ZERO_PAGE: u8 = 0xa6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xb6;
pub const LDX_ABSOLUTE: u8 = 0xae;
pub const LDX_ABSOLUTE_Y: u8 = 0xbe;

pub const LDY_IMMEDIATE: u8 = 0xa0;
pub const LDY_ZERO_PAGE: u8 = 0xa4;
pub const LDY_ZERO_PAGE_X: u8 = 0xb4;
pub const LDY_ABSOLUTE: u8 = 0xac;
pub const LDY_ABSOLUTE_X: u8 = 0xbc;

pub const TAX: u8 = 0xaa;
pub const INX: u8 = 0xe8;
