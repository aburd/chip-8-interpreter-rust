use std::error::Error;
use std::path::Path;
use cpu::Cpu;

mod cpu;

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn main() -> BoxResult<()> {
    // CLI
    let args: Vec<_> = std::env::args().collect();
    let path_str = args.get(1).expect("A path to the rom is needed!");

    // Set up render system and register input callbacks

    // Initialize the Chip8 system and load the game into the memory
    let mut cpu = Cpu::new();
    let rom_path = Path::new(path_str);
    cpu.load_rom(rom_path)?;

    // Emulation loop
    loop {
        // Emulate one cycle
        match cpu.emulate_cycle() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        // If the draw flag is set, update the screen
        if cpu.draw_flag {
            cpu.draw_graphics();
        }

        // Store key press state (Press and Release)
        cpu.set_keys();
    }
}
