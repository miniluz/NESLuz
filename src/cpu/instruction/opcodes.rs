/// Add With Carry (Immediate)
pub const ADC_IMMEDIATE: u8 = 0x69;
/// Add With Carry (Zero page)
pub const ADC_ZERO_PAGE: u8 = 0x65;
/// Add With Carry (Zero page, X)
pub const ADC_ZERO_PAGE_X: u8 = 0x75;
/// Add With Carry (Absolute)
pub const ADC_ABSOLUTE: u8 = 0x6d;
/// Add With Carry (Absolute, X)
pub const ADC_ABSOLUTE_X: u8 = 0x7d;
/// Add With Carry (Absolute, Y)
pub const ADC_ABSOLUTE_Y: u8 = 0x79;
/// Add With Carry (Indirect, X)
pub const ADC_INDIRECT_X: u8 = 0x61;
/// Add With Carry (Indirect, Y)
pub const ADC_INDIRECT_Y: u8 = 0x71;

/// Bit-wise AND (Immediate)
pub const AND_IMMEDIATE: u8 = 0x29;
/// Bit-wise AND (Zero page)
pub const AND_ZERO_PAGE: u8 = 0x25;
/// Bit-wise AND (Zero page, X)
pub const AND_ZERO_PAGE_X: u8 = 0x35;
/// Bit-wise AND (Absolute)
pub const AND_ABSOLUTE: u8 = 0x2d;
/// Bit-wise AND (Absolute, X)
pub const AND_ABSOLUTE_X: u8 = 0x3d;
/// Bit-wise AND (Absolute, Y)
pub const AND_ABSOLUTE_Y: u8 = 0x39;
/// Bit-wise AND (Indirect, X)
pub const AND_INDIRECT_X: u8 = 0x21;
/// Bit-wise AND (Indirect, Y)
pub const AND_INDIRECT_Y: u8 = 0x31;

/// Arithmetic Shift Left (Accumulator)
pub const ASL_ACCUMULATOR: u8 = 0x0a;
/// Arithmetic Shift Left (Zero page)
pub const ASL_ZERO_PAGE: u8 = 0x06;
/// Arithmetic Shift Left (Zero page, X)
pub const ASL_ZERO_PAGE_X: u8 = 0x16;
/// Arithmetic Shift Left (Absolute)
pub const ASL_ABSOLUTE: u8 = 0x0e;
/// Arithmetic Shift Left (Absolute, X)
pub const ASL_ABSOLUTE_X: u8 = 0x1e;

/// Branch if Carry Clear
pub const BCC: u8 = 0x90;
/// Branch if Carry Set
pub const BCS: u8 = 0xb0;
/// Branch if Equal (Zero Set)
pub const BEQ: u8 = 0xf0;

/// Bit test (Zero page)
pub const BIT_ZERO_PAGE: u8 = 0x24;
/// Bit test (Absolute)
pub const BIT_ABSOLUTE: u8 = 0x2c;

/// Branch if Minus (Negative Set)
pub const BMI: u8 = 0x30;
/// Branch if Not Equal (Zero Clear)
pub const BNE: u8 = 0xd0;
/// Branch if Positive (Negative Clear)
pub const BPL: u8 = 0x10;

/// Break
pub const BRK: u8 = 0x00;

/// Branch if Overflow Clear
pub const BVC: u8 = 0x50;
/// Branch if Overflow Set
pub const BVS: u8 = 0x70;

/// Clear Carry Flag
pub const CLC: u8 = 0x18;
/// Clear Decimal Mode
pub const CLD: u8 = 0xd8;
/// Clear Interrupt Disable
pub const CLI: u8 = 0x58;
/// Clear Overflow Flag
pub const CLV: u8 = 0xB8;

/// Compare A (Immediate)
pub const CMP_IMMEDIATE: u8 = 0xc9;
/// Compare A (Zero Page)
pub const CMP_ZERO_PAGE: u8 = 0xc5;
/// Compare A (Zero Page, X)
pub const CMP_ZERO_PAGE_X: u8 = 0xd5;
/// Compare A (Absolute)
pub const CMP_ABSOLUTE: u8 = 0xcd;
/// Compare A (Absolute, X)
pub const CMP_ABSOLUTE_X: u8 = 0xdd;
/// Compare A (Absolute, Y)
pub const CMP_ABSOLUTE_Y: u8 = 0xd9;
/// Compare A (Indirect, X)
pub const CMP_INDIRECT_X: u8 = 0xc1;
/// Compare A (Indirect, Y)
pub const CMP_INDIRECT_Y: u8 = 0xd1;

/// Compare X (Immediate)
pub const CPX_IMMEDIATE: u8 = 0xe0;
/// Compare X (Zero page)
pub const CPX_ZERO_PAGE: u8 = 0xe4;
/// Compare X (Absolute)
pub const CPX_ABSOLUTE: u8 = 0xec;

/// Compare Y (Immediate)
pub const CPY_IMMEDIATE: u8 = 0xc0;
/// Compare Y (Zero page)
pub const CPY_ZERO_PAGE: u8 = 0xc4;
/// Compare Y (Absolute)
pub const CPY_ABSOLUTE: u8 = 0xcc;

/// Load to Accumulator (Immediate)
pub const LDA_IMMEDIATE: u8 = 0xa9;
/// Load to Accumulator (Zero page)
pub const LDA_ZERO_PAGE: u8 = 0xa5;
/// Load to Accumulator (Zero page, X)
pub const LDA_ZERO_PAGE_X: u8 = 0xb5;
/// Load to Accumulator (Absolute)
pub const LDA_ABSOLUTE: u8 = 0xad;
/// Load to Accumulator (Absolute, X)
pub const LDA_ABSOLUTE_X: u8 = 0xbd;
/// Load to Accumulator (Absolute, Y)
pub const LDA_ABSOLUTE_Y: u8 = 0xb9;
/// Load to Accumulator (Indirect, X)
pub const LDA_INDIRECT_X: u8 = 0xa1;
/// Load to Accumulator (Indirect, Y)
pub const LDA_INDIRECT_Y: u8 = 0xb1;

/// Load to X (Immediate)
pub const LDX_IMMEDIATE: u8 = 0xa2;
/// Load to X (Zero page)
pub const LDX_ZERO_PAGE: u8 = 0xa6;
/// Load to X (Zero page, Y)
pub const LDX_ZERO_PAGE_Y: u8 = 0xb6;
/// Load to X (Absolute)
pub const LDX_ABSOLUTE: u8 = 0xae;
/// Load to X (Absolute, Y)
pub const LDX_ABSOLUTE_Y: u8 = 0xbe;

/// Load to Y (Immediate)
pub const LDY_IMMEDIATE: u8 = 0xa0;
/// Load to Y (Zero page)
pub const LDY_ZERO_PAGE: u8 = 0xa4;
/// Load to Y (Zero page, X)
pub const LDY_ZERO_PAGE_X: u8 = 0xb4;
/// Load to Y (Absolute)
pub const LDY_ABSOLUTE: u8 = 0xac;
/// Load to Y (Absolute, X)
pub const LDY_ABSOLUTE_X: u8 = 0xbc;

/// Transfer Accumulator to X
pub const TAX: u8 = 0xaa;

/// Increment X
pub const INX: u8 = 0xe8;
