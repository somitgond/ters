use std::io::Read;
use std::{io, os::fd::AsRawFd};
// use std::os::unix::io::RawFd;
use std::io::{Write, stdout};
use termios::*;

// enable terminal raw mode
fn enable_raw_mode() -> Result<Termios, std::io::Error> {
    let stdin = io::stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin)?;
    let termios_orig = termios.clone();

    termios.c_cflag |= CS8;
    termios.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    termios.c_oflag &= !OPOST;
    termios.c_iflag &= !(BRKINT | ISTRIP | INLCR | ICRNL | IXON);

    termios.c_cc[VMIN] = 0;
    termios.c_cc[VTIME] = 1;
    let _ = tcsetattr(stdin, TCSAFLUSH, &mut termios);

    Ok(termios_orig)
}

fn disable_raw_mode(mut termios_orig: &Termios) {
    let stdin = io::stdin().as_raw_fd();
    let _ = tcsetattr(stdin, TCSAFLUSH, &mut termios_orig);
}

fn write(s: &str) {
    let mut lock = stdout().lock();
    write!(lock, "{}", s).unwrap();
    stdout().flush().unwrap();
}

fn clear_screen() {
    write("\x1b[2J");
}

// main function
fn main() {
    let mut termios_orig = enable_raw_mode().unwrap();

    clear_screen();

    'outer_loop: loop {
        // read bytes from stdin
        for i in io::stdin().bytes() {
            let c: char = i.unwrap() as char;

            write(&c.to_string());

            if c == 'q' {
                break 'outer_loop;
            }
        }
    }
    disable_raw_mode(&mut termios_orig);
}
