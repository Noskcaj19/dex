pub extern crate termion;
extern crate unicode_segmentation;
extern crate unicode_width;

use std::io::{stdout, Error, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Copy, Clone)]
pub struct TermSize {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct TermCell {
    pub ch: String,
}

pub struct TermBuf {
    pub terminal: AlternateScreen<RawTerminal<Stdout>>,
    size: TermSize,
    show_cursor: bool,
    cursor_pos: (u16, u16),
    buf: Vec<Vec<Option<TermCell>>>,
}

impl TermBuf {
    /// Constructs a new TermBuf instance and enters the terminal alternate screen
    pub fn init() -> Result<TermBuf, Error> {
        let rawsize = termion::terminal_size()?;
        let size = TermSize {
            width: rawsize.0,
            height: rawsize.1,
        };

        let buf = vec![vec![None; size.width as usize]; size.height as usize];
        Ok(TermBuf {
            terminal: AlternateScreen::from(stdout().into_raw_mode()?),
            buf,
            show_cursor: true,
            cursor_pos: (1, 1),
            size,
        })
    }

    /// Call this when the terminal changes size, the internal buffer will be resized
    pub fn update_size(&mut self) -> Result<(), Error> {
        let new_size = self.size()?;
        self.size = new_size;
        let mut new_buf = vec![vec![None; new_size.width as usize]; new_size.height as usize];
        {
            for (r, row) in self.buf.iter().enumerate() {
                for (c, ch) in row.iter().enumerate() {
                    new_buf
                        .get_mut(r)
                        .map(|row| row.get_mut(c).map(|new_ch| *new_ch = ch.to_owned()));
                }
            }
        }
        self.buf = new_buf;
        Ok(())
    }

    /// Draw internal buffer to the terminal
    pub fn draw(&mut self) -> Result<(), Error> {
        write!(self.terminal, "{}", termion::clear::All)?;

        for (row, text) in self.buf.iter().enumerate() {
            eprintln!(
                "{:?}",
                text.into_iter()
                    .map(|cell| if let Some(cell) = cell {
                        cell.ch.to_owned()
                    } else {
                        " ".to_owned()
                    })
                    .collect::<String>()
                    .trim_right()
            );
            write!(
                self.terminal,
                "{}{}",
                termion::cursor::Goto(1, row as u16),
                text.into_iter()
                    .map(|cell| if let Some(cell) = cell {
                        cell.ch.to_owned()
                    } else {
                        " ".to_owned()
                    })
                    .collect::<String>()
                    .trim_right()
            )?;
        }

        if self.show_cursor {
            write!(
                self.terminal,
                "{}",
                termion::cursor::Goto(self.cursor_pos.0, self.cursor_pos.1)
            )?;
        }
        self.terminal.flush()?;
        Ok(())
    }

    /// Sets cursor visiblity
    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.show_cursor = visible;
        if visible {
            write!(self.terminal, "{}", termion::cursor::Show);
        } else {
            write!(self.terminal, "{}", termion::cursor::Hide);
        }
    }

    /// Sets cursor position
    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.cursor_pos = (x, y);
    }

    /// Gets size of the terminal
    pub fn size(&self) -> Result<TermSize, Error> {
        let rawsize = termion::terminal_size()?;
        Ok(TermSize {
            width: rawsize.0,
            height: rawsize.1,
        })
    }

    /// Writes a single char
    pub fn set_char(&mut self, ch: char, x: u16, y: u16) {
        self.set_cell(TermCell { ch: ch.to_string() }, x, y)
    }

    fn set_cell(&mut self, cell: TermCell, x: u16, y: u16) {
        if let Some(line) = self.buf.get_mut(y as usize) {
            if let Some(mut old_ch) = line.get_mut(x as usize) {
                *old_ch = Some(cell);
            }
        }
    }

    /// Writes an entire string
    pub fn put_string(&mut self, string: &str, mut x: u16, mut y: u16) {
        for ch in UnicodeSegmentation::graphemes(string, true) {
            match ch {
                "\n" => y += 1,
                _ => {
                    self.set_cell(TermCell { ch: ch.to_owned() }, x, y);
                    // x += UnicodeWidthStr::width(ch) as u16;
                    x += 1;
                }
            }
        }
    }

    /// Draws a unicode box
    pub fn draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) {
        let width = width + 1;
        let height = height + 1;
        self.set_char('┌', x, y);
        self.set_char('┐', x + width, y);
        self.set_char('└', x, y + height);
        self.set_char('┘', x + width, y + height);

        for i in (x + 1)..(width + x) {
            self.set_char('─', i, y);
            self.set_char('─', i, y + height);
        }

        for i in y + 1..height + y {
            self.set_char('│', x, i);
            self.set_char('│', x + width, i);
        }
    }

    /// Empties buffer
    pub fn clear(&mut self) {
        let size = self.size;
        self.buf = vec![vec![None; size.width as usize]; size.height as usize];
    }
}
