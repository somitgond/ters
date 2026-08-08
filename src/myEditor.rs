// Editor Struct and related functions
use std::fs;
use std::io::{Read, Write, stdin, stdout};
use std::os::fd::AsRawFd;
use termios::*;

// EditorState: Structure to store termios and other states
pub struct EditorState {
    pub termios_orig: Termios,
    pub rows: i32,             // total rows in the terminal
    pub cols: i32,             // total columns in the terminal
    pub buffer_text: String,   // buffer text to display
    pub cx: i32,               // current row value
    pub cy: i32,               // current cols value
    pub num_rows: i32,         // number of rows of data available
    pub row_data: String,      // data per row, FIXME: should be vector of num_rows
    pub status_string: String, // status line string
}

impl EditorState {
    // enable terminal raw mode
    pub fn enable_raw_mode(&self) {
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
    pub fn disable_raw_mode(&mut self) {
        let stdin_fd = stdin().as_raw_fd();
        let _ = tcsetattr(stdin_fd, TCSANOW, &mut self.termios_orig);
    }

    // get window size
    pub fn get_window_size(&mut self) {
        write("\x1b[999C\x1b[999B\x1b[6n\r\n");

        // has semicolon has been encountered
        let mut semicolon = false;
        let mut rows: i32 = 0;
        let mut cols: i32 = 0;

        loop {
            // read 1 byte at a time
            let mut buf = [0; 1];
            let _ = stdin().read_exact(&mut buf).unwrap();

            match buf[0] {
                82 => break,
                59 => semicolon = true,
                48..57 => {
                    if !semicolon {
                        rows = rows * 10 + (buf[0] - 48) as i32;
                    } else if semicolon {
                        cols = cols * 10 + (buf[0] - 48) as i32;
                    }
                }
                _ => (),
            };
        }
        let status_str = format!("Rows: {rows}, cols: {cols}").to_string();
        self.status_string = status_str;

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

pub fn write(s: &str) {
    let mut lock = stdout().lock();
    write!(lock, "{}", s).unwrap();
    stdout().flush().unwrap();
}

pub fn clear_screen() {
    write("\x1b[2J");
}

pub fn reposition_cursor(global_state: &EditorState) {
    let mut s = String::from("\x1b[");
    s.push_str(&(global_state.cy + 1).to_string());
    s.push_str(";");
    s.push_str(&(global_state.cx + 1).to_string());
    s.push_str("H");

    write(&s);
}

pub fn draw_rows(global_state: &mut EditorState) {
    let mut buffer_text = String::new();
    for y in 0..global_state.rows {
        // if row data exists, show data first
        if y >= global_state.num_rows {
            // show editor version at startup
            if y == global_state.rows / 2 {
                let s = String::from("Text Editor in RuSt -- version 0.1");
                // center the welcome message
                let slen: i32 = s.len() as i32;
                buffer_text.push('~');
                if slen < global_state.cols {
                    for _ in 0..(global_state.cols - slen) / 2 - 1 {
                        buffer_text.push(' ');
                    }
                }
                buffer_text.push_str(&s);
                buffer_text.push_str(&format!("cols: {}, cols: {}", global_state.rows, global_state.cols));
            } else {
                buffer_text.push('~');
            }
        } else {
            let mut len = global_state.row_data.len() as i32;

            // if data length is > than cols
            if len > global_state.cols {
                len = global_state.cols;
            }

            buffer_text.push_str(&global_state.row_data[0..len as usize]);
        }

        buffer_text.push_str("\x1b[K");

        // tilde on last line also
        if y < global_state.rows - 1 {
            buffer_text.push_str("\r\n");
        }
    }
    write(&buffer_text);
}

pub fn hide_cursor() {
    write("\x1b[?25l");
}

pub fn show_cursor() {
    write("\x1b[?25h");
}

pub fn run_editor(global_state: &mut EditorState) {
    // enable raw mode
    global_state.enable_raw_mode();
    global_state.get_window_size();

    clear_screen();
    hide_cursor();
    global_state.get_window_size();
    draw_rows(global_state);
    reposition_cursor(global_state);
    show_cursor();

    'outer_loop: loop {
        for i in stdin().bytes() {
            let c: char = i.unwrap() as char;

            match c {
                'q' => break 'outer_loop,
                _ => global_state.buffer_text.push(c),
            }
            //hide_cursor();
            global_state.get_window_size();
            draw_rows(global_state);
            reposition_cursor(global_state);
            write(&(c.to_string()));
            global_state.row_data.push(c);
            //show_cursor();
        }
    }
}

// open file and fill row values
pub fn editor_open(filename: &str, global_state: &mut EditorState) {
    let contents = fs::read_to_string(filename).expect("Invalid file");

    global_state.row_data = contents[0..10].to_string();
    global_state.num_rows = 1;
}

// create editor state
pub fn create_editor_state() -> EditorState {
    let stdin_fd = stdin().as_raw_fd();

    EditorState {
        termios_orig: Termios::from_fd(stdin_fd).unwrap(),
        rows: 0,
        cols: 0,
        buffer_text: String::new(),
        cx: 0,
        cy: 0,
        num_rows: 0,
        row_data: String::new(),
        status_string: String::new(),
    }
}
