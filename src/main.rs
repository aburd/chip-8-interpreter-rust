use std::error::Error;
use std::path::Path;
mod interpreter;

use interpreter::Chip8Interpreter;

type BoxResult<T> = Result<T, Box<dyn Error>>;

fn main() -> BoxResult<()> {
    // CLI
    let args: Vec<_> = std::env::args().collect();
    let path_str = args.get(1).expect("A path to the rom is needed!");

    // Set up render system and register input callbacks

    // Initialize the Chip8 system and load the game into the memory
    let mut chip8_interpreter = Chip8Interpreter::new();
    let rom_path = Path::new(path_str);
    chip8_interpreter.load_rom(rom_path)?;

    // Emulation loop
    loop {
        // Emulate one cycle
        match chip8_interpreter.emulate_cycle() {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        // If the draw flag is set, update the screen
        if chip8_interpreter.draw_flag {
            chip8_interpreter.draw_graphics();
        }

        // Store key press state (Press and Release)
        chip8_interpreter.set_keys();
    }
}
