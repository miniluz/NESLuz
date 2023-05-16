pub struct Memory {
    memory: [u8; 0xFFFF],
}

impl std::fmt::Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("[...]")
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF],
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

    pub fn write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn load(&mut self, address: u16, data: &[u8]) {
        let address = address as usize;
        self.memory[address..(address + data.len())].copy_from_slice(data);
    }
}
