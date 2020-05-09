mod interpreter;

use interpreter::Chip8Interpreter;

fn main() {
    // Set up render system and register input callbacks

    // Initialize the Chip8 system and load the game into the memory
    let mut chip8_interpreter = Chip8Interpreter::new();

    // Emulation loop
    loop {
        // Emulate one cycle
        chip8_interpreter.emulate_cycle();

        // If the draw flag is set, update the screen
        if chip8_interpreter.draw_flag {
            chip8_interpreter.draw_graphics();
        }

        // Store key press state (Press and Release)
        chip8_interpreter.set_keys();
    }
}
