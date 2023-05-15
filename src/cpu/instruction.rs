#[cfg(test)]
mod test {
    use crate::cpu::instruction::*;

    #[test]
    fn lda() {
        let instructions = vec![0xa9, 0xc0];
        let result = Instruction::get_instruction(&instructions, &0).unwrap();
        dbg!(result);
        assert!(matches!(
            Instruction::get_instruction(&instructions, &0).unwrap(),
            (Instruction::LDA(LDAType::Immediate { immediate: 0xc0 }), 2)
        ));
    }
}

#[derive(Debug)]
pub enum LDAType {
    Immediate { immediate: u8 },
}

#[derive(Debug)]
pub enum Instruction {
    LDA(LDAType),
}

fn null_pointer() -> color_eyre::Report {
    return color_eyre::eyre::eyre!("Should have instruction at pointer.");
}

impl Instruction {
    pub fn get_instruction(
        instructions: &[u8],
        offset: &u16,
    ) -> color_eyre::Result<(Instruction, u16)> {
        let mut instructions = instructions.iter().skip(*offset as usize).enumerate();
        let (offset, instruction) = match instructions.next().ok_or(null_pointer())? {
            (_, 0xa9) => {
                let (offset, &immediate) = instructions.next().ok_or(null_pointer())?;
                (offset, Instruction::LDA(LDAType::Immediate { immediate }))
            }
            _ => {
                return Err(color_eyre::eyre::eyre!(
                    "Should have been able to find instruction!"
                ));
            }
        };

        Ok((instruction, u16::try_from(offset)? + 1))
    }
}
