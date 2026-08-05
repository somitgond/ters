use std::os::fd::AsRawFd;
use std::io::{Read, Write, stdout, stdin};
use termios::*;

// EditorState: Structure to store termios and other states
struct EditorState {
    termios_orig : Termios,
    rows: i32,
    cols: i32,
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
        let _ = tcsetattr(stdin_fd, TCSANOW, &mut termios);
    }

    // disable terminal raw mode
    fn disable_raw_mode(&mut self) {
        let stdin_fd = stdin().as_raw_fd();
        let _ = tcsetattr(stdin_fd, TCSANOW, &mut self.termios_orig);
    }

    // get window size
    fn get_window_size(&mut self) {
        write("\x1b[999C\x1b[999B\x1b[6n\r\n");
        // write("\x1b[6n\r\n");
        let mut nCharsRead = 0;
        // FIXME; based on terminal width/height buffer size can increase/decrease
        let mut buffer = [0; 9];
        let _ = stdin().read_exact(&mut buffer).unwrap();
        let mut s = String::new();
        let mut semicolon = false;
        let mut sq = false;
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        for i in buffer {
            if i == 82 {
                break;
            } else if i == 59 {
                semicolon = true;
            } else if i == 91 {
                sq = true;
            } else if sq && !semicolon {
                width = width *10 + (i - 48) as i32;
            } else if sq && semicolon {
                height = height*10 + (i - 48) as i32;
            }
        }

        self.rows = width;
        self.cols = height;
        // println!("w: {}, h: {}", width, height);
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

fn draw_rows(global_state: &EditorState) {
    for i in 0..global_state.rows {
        write("~\r\n");
    }
}

fn run_editor(global_state: &EditorState) {
    clear_screen();
    reposition_cursor();
    draw_rows(global_state);
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
        rows: 0,
        cols: 0,
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
