use std::env;

mod my_editor;
use my_editor::*;

mod my_logger;

fn main() {
    // Load termios
    let mut global_state = create_editor_state();

    // command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Insufficient number of argument specified");
        editor_open(&mut global_state);
    } else {
        editor_open_file(&args[1], &mut global_state);
    }

    // finally run editor main loop
    run_editor(&mut global_state);
}

// FIXME:
// 1. Disable raw mode at program exit (in case of panic and normal exit)
//    - Not found a reliable way to disable raw mode in case of panic
