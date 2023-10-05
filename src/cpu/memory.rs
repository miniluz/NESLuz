use thiserror::Error;

#[derive(Debug)]
pub struct Address {
    address: u16,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load() {
        let mut memory = Memory::new();
        memory.load(0x8000, &[0x01, 0x02, 0x03, 0x04]).unwrap();
        assert_eq!(memory.memory[..0x8000], [0; 0x8000]);
        assert_eq!(memory.memory[0x8000..0x8004], [0x01, 0x02, 0x03, 0x04]);
        assert_eq!(memory.memory[0x8004..], [0; 0x8000 - 4 - 1]);
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
}

#[derive(Error, Debug)]
pub enum CpuMemoryError {
    #[error("memory address ({address}) out of bounds")]
    IndexOutOfBounds { address: u16 },
}

pub struct Memory {
    memory: [u8; 0x10000],
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("[...]")
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0x10000],
        }
    }

    pub fn iter(&self) -> std::slice::Iter<u8> {
        self.memory.iter()
    }
}

impl Memory {
    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        let lo = self.read(address) as u16;
        let hi = self.read(address.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn write(&mut self, address: u16, data: u8) -> () {
        self.memory[address as usize] = data;
    }

    pub fn write_u16(&mut self, address: u16, data: u16) -> () {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.write(address, lo);
        self.write(address.wrapping_add(1), hi);
    }

    pub fn load(&mut self, address: u16, data: &[u8]) -> Result<(), CpuMemoryError> {
        let address = address as usize;
        self.memory
            .get_mut(address..(address + data.len()))
            .ok_or(CpuMemoryError::IndexOutOfBounds {
                address: address as u16,
            })?
            .copy_from_slice(data);
        Ok(())
    }
}
