mod instruction;

use instruction::*;

#[derive(Debug)]
pub struct CPU {
    register_a: u8,
    status: u8,
    program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0x0,
            status: 0x0,
            program_counter: 0x0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) -> color_eyre::Result<()> {
        self.program_counter = 0;

        loop {
            let (instruction, offset) =
                Instruction::get_instruction(&program, &self.program_counter)?;
            self.program_counter += 2;

            use Instruction::*;

            match instruction {
                LDA(lda) => match lda {
                    LDAType::Immediate { immediate } => {
                        self.register_a = immediate;
                    }
                },
            }
        }

        Ok(())
    }
}
