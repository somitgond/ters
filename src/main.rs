use std::os::fd::AsRawFd;
use std::io::{Read, Write, stdout, stdin};
use termios::*;

// EditorState: Structure to store termios and other states
struct EditorState {
    termios_orig : Termios,
    rows: i32,
    cols: i32,
    buffer_text: String,
    cx: i32,
    cy: i32,
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
        let mut semicolon = false;
        let mut sq = false;
        let mut rows: i32 = 0;
        let mut cols: i32 = 0;

        loop {
            // read 1 byte at a time
            let mut buf = [0;1];
            let _ = stdin().read_exact(&mut buf).unwrap();

            if buf[0] == 82 {
                break;
            } else if buf[0] == 59 {
                semicolon = true;
            } else if buf[0] == 91 {
                sq = true;
            } else if sq && !semicolon {
                rows = rows *10 + (buf[0] - 48) as i32;
            } else if sq && semicolon {
                cols = cols*10 + (buf[0] - 48) as i32;
            }
        }

        self.rows = rows;
        self.cols = cols;
    }
}

// implement drop trait
impl Drop for EditorState {
    fn drop(&mut self) {
        // clear screen and position cursor at top
        clear_screen();
        self.cx = 0;
        self.cy = 0;
        reposition_cursor(self);
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

fn reposition_cursor(global_state: &EditorState) {

    let mut s = String::from("\x1b[");
    s.push_str(&(global_state.cy +1).to_string());
    s.push_str(";");
    s.push_str(&(global_state.cx +1).to_string());
    s.push_str("H");

    write(&s);
}

fn draw_rows(global_state: &mut EditorState) {

    for i in 0..global_state.rows {

        // show editor version at startup
        if i == global_state.rows/2 {
            let s = String::from("Text Editor in RuSt -- version 0.1");
            // center the welcome message
            let slen: i32 = s.len() as i32;
            global_state.buffer_text.push('~');
            if slen < global_state.cols {
                for _ in 0..(global_state.cols - slen)/2-1 {
                    global_state.buffer_text.push(' ');
                }
            }
            global_state.buffer_text.push_str(&s); // erase a line at a time
        } else {
            global_state.buffer_text.push('~');
        }

        global_state.buffer_text.push_str("\x1b[K"); // erase a line at a time

        // tilde on last line also
        if i < global_state.rows-1 {
            global_state.buffer_text.push_str("\r\n");
        }
    }
    write(&global_state.buffer_text);
}

fn hide_cursor() {
    write("\x1b[?25l");
}

fn show_cursor() {
    write("\x1b[?25h");
}

fn run_editor(global_state: &mut EditorState) {
    clear_screen();
    reposition_cursor(global_state);
    hide_cursor();
    draw_rows(global_state);
    reposition_cursor(global_state);
    show_cursor();

    'outer_loop: loop {
        for i in stdin().bytes() {
            let c: char = i.unwrap() as char;

            match c {
                'A' => global_state.cx+=1,
                'E' => global_state.cy+=1,
                'q' => break 'outer_loop,
                _ => write(&c.to_string()),
            }
            reposition_cursor(global_state);
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
        buffer_text: String::new(),
        cx: 0,
        cy: 0,
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
