use std::{io, os::fd::AsRawFd};
use std::io::Read;
// use std::os::unix::io::RawFd;
use termios::*;

// enable terminal raw mode
fn enable_raw_mode() -> Result<Termios, std::io::Error> {
  let stdin = io::stdin().as_raw_fd();
  let mut termios = Termios::from_fd(stdin)?;
  let termios_orig = termios.clone();

  termios.c_cflag |= CS8;
  // termios.c_cflag &= !(CSIZE | PARENB);
  //termios.c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
  termios.c_lflag &= !(ECHO | ICANON | ISIG | IEXTEN);
  termios.c_oflag &= !OPOST;
  //termios.c_iflag &= !(IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
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

// main function
fn main() {
    let termios_orig = enable_raw_mode().unwrap();

    loop {
        // read chars 1 byte at a time
        for i in io::stdin().bytes() {
            let c: char = i.unwrap() as char;
            print!("{}", c);
            if c == 'q' {
                break;
            }
        }
    }
    disable_raw_mode(&mut termios_orig);
}
