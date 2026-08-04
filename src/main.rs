use std::os::fd::AsRawFd;
use std::io::{Read, Write, stdout, stdin};
use termios::*;

// EditorState: Structure to store termios and other states
struct EditorState {
    termios_orig : Termios,
    width: i32,
    height: i32,
}

impl EditorState {
    // enable terminal raw mode
    fn enable_raw_mode(&self) {
        let mut termios = self.termios_orig.clone();
        let stdin_fd = stdin().as_raw_fd();

        termios.c_cflag |= CS8;
        termios.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
        termios.c_oflag &= !OPOST;
        termios.c_iflag &= !(BRKINT | ISTRIP | INLCR | ICRNL | IXON);

        termios.c_cc[VMIN] = 0;
        termios.c_cc[VTIME] = 1;
        let _ = tcsetattr(stdin_fd, TCSAFLUSH, &mut termios);
    }

    // disable terminal raw mode
    fn disable_raw_mode(&mut self) {
        let stdin_fd = stdin().as_raw_fd();
        let _ = tcsetattr(stdin_fd, TCSAFLUSH, &mut self.termios_orig);
    }

    // get window size
    fn get_window_size(&mut self) {
        write("\x1b[999C\x1b[999B");
        write("\x1b[6n");
        write("\r\n");
        for _ in 0..32 {
            for i in stdin().bytes() {
                let c: char = i.unwrap() as char;
                write(&c.to_string());
            }
        }
    }
}

// implement drop trait
impl Drop for EditorState {
    fn drop(&mut self) {
        // clear screen and position cursor at top
        clear_screen();
        reposition_cursor();
        self.disable_raw_mode();
    }
}

fn write(s: &str) {
    let mut lock = stdout().lock();
    write!(lock, "{}", s).unwrap();
    stdout().flush().unwrap();
}

fn clear_screen() {
    write("\x1b[2J");
}

fn reposition_cursor() {
    write("\x1b[H");
}

fn draw_rows() {
    for i in 0..30 {
        write("~\r\n");
    }
}

fn run_editor(global_state: &EditorState) {
    clear_screen();
    reposition_cursor();
    draw_rows();
    reposition_cursor();

    'outer_loop: loop {
        for i in stdin().bytes() {
            let c: char = i.unwrap() as char;

            write(&c.to_string());

            if c == 'q' {
                break 'outer_loop;
            }
        }
    }
}

// main function
fn main() {
    let stdin_fd = stdin().as_raw_fd();

    // Load termios
    let mut global_state: EditorState  = EditorState {
        termios_orig : Termios::from_fd(stdin_fd).unwrap(),
        height: 0,
        width: 0,
    };

    // enable raw mode
    global_state.enable_raw_mode();
    global_state.get_window_size();

    // finally run editor main loop
    run_editor(&mut global_state);
}


// FIXME:
// 1. Disable raw mode at program exit (in case of panic and normal exit)
// 2. Logger
