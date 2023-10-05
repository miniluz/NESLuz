use super::*;

#[test]
fn load() {
    let mut memory = Memory::new();
    memory.load(0x8000, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    assert_eq!(memory.memory[..0x8000], [0; 0x8000]);
    assert_eq!(memory.memory[0x8000..0x8004], [0x01, 0x02, 0x03, 0x04]);
    assert_eq!(memory.memory[0x8004..], [0; 0x8000 - 4]);
}

#[test]
fn read() {
    let mut memory = Memory::new();
    memory.load(0x8000, &[0x01, 0x02, 0x03, 0x04]).unwrap();
    assert!(matches!(memory.read(0x8000), 0x01));
    assert!(matches!(memory.read(0x8001), 0x02));
    assert!(matches!(memory.read(0x8002), 0x03));
    assert!(matches!(memory.read(0x8003), 0x04));
    assert!(matches!(memory.read_u16(0x8000), 0x0201));
    assert!(matches!(memory.read_u16(0x8002), 0x0403));
}

#[test]
fn write() {
    let mut memory = Memory::new();
    memory.write(0x8000, 0x01);
    memory.write(0x8001, 0x02);
    memory.write_u16(0x8002, 0x0403);
    assert_eq!(memory.memory[0x8000..0x8004], [0x01, 0x02, 0x03, 0x04]);
}
