use std::env;
use std::process::exit;

mod myEditor;
use myEditor::*;

mod myLogger;

fn main() {
    // command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Insufficient number of argument specified");
        exit(0);
    }

    // Load termios
    let mut global_state = create_editor_state();
    editor_open(&args[1], &mut global_state);

    // finally run editor main loop
    run_editor(&mut global_state);
}

// FIXME:
// 1. Disable raw mode at program exit (in case of panic and normal exit)
// 2. Logger
