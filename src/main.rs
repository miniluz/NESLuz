use derives::AddressingEnum;

pub mod cpu;

fn main() -> color_eyre::Result<()> {
    #[derive(AddressingEnum)]
    enum _Gente {
        #[modes(mode = "immediate", mode = "zero_page")]
        Juan,
        #[modes(mode = "immediate", mode = "zero_page")]
        Pepe,
    }

    Ok(())
}
