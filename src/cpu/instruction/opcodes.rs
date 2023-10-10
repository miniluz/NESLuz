// Add With Carry
pub const ADC_IMMEDIATE: u8 = 0x69;
pub const ADC_ZERO_PAGE: u8 = 0x65;
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
pub const ADC_ABSOLUTE: u8 = 0x6d;
pub const ADC_ABSOLUTE_X: u8 = 0x7d;
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
pub const ADC_INDIRECT_X: u8 = 0x61;
pub const ADC_INDIRECT_Y: u8 = 0x71;

// Bit-wise AND
pub const AND_IMMEDIATE: u8 = 0x29;
pub const AND_ZERO_PAGE: u8 = 0x25;
pub const AND_ZERO_PAGE_X: u8 = 0x35;
pub const AND_ABSOLUTE: u8 = 0x2d;
pub const AND_ABSOLUTE_X: u8 = 0x3d;
pub const AND_ABSOLUTE_Y: u8 = 0x39;
pub const AND_INDIRECT_X: u8 = 0x21;
pub const AND_INDIRECT_Y: u8 = 0x31;

// Arithmetic Shift Left
pub const ASL_ACCUMULATOR: u8 = 0x0a;
pub const ASL_ZERO_PAGE: u8 = 0x06;
pub const ASL_ZERO_PAGE_X: u8 = 0x16;
pub const ASL_ABSOLUTE: u8 = 0x0e;
pub const ASL_ABSOLUTE_X: u8 = 0x1e;

// Branch if Carry Clear
pub const BCC: u8 = 0x90;
// Branch if Carry Set
pub const BCS: u8 = 0xb0;
// Branch if Equal (Zero Set)
pub const BEQ: u8 = 0xf0;

// Bit test
pub const BIT_ZERO_PAGE: u8 = 0x24;
pub const BIT_ABSOLUTE: u8 = 0x2c;

// Branch if Minus (Negative Set)
pub const BMI: u8 = 0x30;
// Branch if Not Equal (Zero Clear)
pub const BNE: u8 = 0xd0;
// Branch if Positive (Negative Clear)
pub const BPL: u8 = 0x10;

// Break
pub const BRK: u8 = 0x00;

// Branch if Overflow Clear
pub const BVC: u8 = 0x50;

// Load accumulator
pub const LDA_IMMEDIATE: u8 = 0xa9;
pub const LDA_ZERO_PAGE: u8 = 0xa5;
pub const LDA_ZERO_PAGE_X: u8 = 0xb5;
pub const LDA_ABSOLUTE: u8 = 0xad;
pub const LDA_ABSOLUTE_X: u8 = 0xbd;
pub const LDA_ABSOLUTE_Y: u8 = 0xb9;
pub const LDA_INDIRECT_X: u8 = 0xa1;
pub const LDA_INDIRECT_Y: u8 = 0xb1;

// Load X register
pub const LDX_IMMEDIATE: u8 = 0xa2;
pub const LDX_ZERO_PAGE: u8 = 0xa6;
pub const LDX_ZERO_PAGE_Y: u8 = 0xb6;
pub const LDX_ABSOLUTE: u8 = 0xae;
pub const LDX_ABSOLUTE_Y: u8 = 0xbe;

// Load Y register
pub const LDY_IMMEDIATE: u8 = 0xa0;
pub const LDY_ZERO_PAGE: u8 = 0xa4;
pub const LDY_ZERO_PAGE_X: u8 = 0xb4;
pub const LDY_ABSOLUTE: u8 = 0xac;
pub const LDY_ABSOLUTE_X: u8 = 0xbc;

// Transfer Accumulator to X
pub const TAX: u8 = 0xaa;

// Increment X register
pub const INX: u8 = 0xe8;
