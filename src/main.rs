mod cpu;

fn main() -> color_eyre::Result<()> {
    let mut cpu = cpu::Cpu::new();
    let instructions = vec![0xa9, 0xc0, 0x00];

    cpu.load_and_run(instructions)?;

    Ok(())
}
