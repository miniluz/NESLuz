use derives::AddressingEnum;

pub mod cpu;

fn main() -> color_eyre::Result<()> {
    #[derive(AddressingEnum)]
    #[modes(mode = "accumulator", mode = "zero_page")]
    struct hey {}

    Ok(())
}
