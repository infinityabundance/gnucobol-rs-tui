#![forbid(unsafe_code)]
//! # gnucobol-rs-tui
//!
//! A COBOL **Screen Section** primitive: a 1-based `LINE`/`COL` terminal cell matrix that places `DISPLAY`
//! fields, and renders numeric fields through the **oracle-proven** [`gnucobol-rs`] edited-encode court
//! (`GNURUST.16C`) so a `PIC $$,$$9.99CR` field on screen carries the exact cobc-faithful presentation bytes.
//!
//! Screen positioning itself is a from-scratch primitive (cobc's `screenio.c` is not yet a sealed court);
//! the numeric/edited *content* it places IS court-backed. Faithful-port satellite; LGPL-3.0-or-later.
//!
//! ```
//! use gnucobol_rs_tui::Screen;
//! use gnucobol_rs::Decimal;
//! let mut s = Screen::new(2, 12);
//! s.put(1, 1, b"BALANCE:");
//! s.put_edited(2, 1, &Decimal { negative: true, digits: vec![1,2,5], scale: 1 }, "$$,$$9.99CR").unwrap();
//! assert_eq!(s.line_str(2).trim_end(), "   $12.50CR");
//! ```

use gnucobol_rs::{encode_edited, Decimal, EditedError};

/// A fixed-size terminal cell matrix, space-filled, addressed by 1-based COBOL `LINE`/`COL`.
#[derive(Clone)]
pub struct Screen {
    rows: usize,
    cols: usize,
    cells: Vec<u8>,
}

impl Screen {
    /// A new `rows × cols` screen, space-filled.
    pub fn new(rows: usize, cols: usize) -> Self {
        Screen { rows, cols, cells: vec![b' '; rows * cols] }
    }

    /// Place raw bytes at 1-based (`line`, `col`), clipping at the right edge and ignoring an out-of-range
    /// line (a Screen Section field never wraps or grows the screen).
    pub fn put(&mut self, line: usize, col: usize, text: &[u8]) {
        if line == 0 || col == 0 || line > self.rows {
            return;
        }
        let base = (line - 1) * self.cols + (col - 1);
        for (k, &b) in text.iter().enumerate() {
            let c = (col - 1) + k;
            if c >= self.cols {
                break; // clip at the right edge
            }
            self.cells[base + k] = b;
        }
    }

    /// Place a numeric `value` formatted by `pic` at (`line`, `col`), using the oracle-proven edited-encode
    /// court (`GNURUST.16C`). Returns the [`EditedError`] if the picture is outside the admitted subset.
    pub fn put_edited(
        &mut self,
        line: usize,
        col: usize,
        value: &Decimal,
        pic: &str,
    ) -> Result<(), EditedError> {
        let bytes = encode_edited(pic, value)?;
        self.put(line, col, &bytes);
        Ok(())
    }

    /// The raw bytes of 1-based `line` (space-padded to the full width); empty if out of range.
    pub fn line(&self, line: usize) -> &[u8] {
        if line == 0 || line > self.rows {
            return &[];
        }
        let start = (line - 1) * self.cols;
        &self.cells[start..start + self.cols]
    }

    /// [`Screen::line`] as a `String` (Latin-1).
    pub fn line_str(&self, line: usize) -> String {
        self.line(line).iter().map(|&b| b as char).collect()
    }

    /// The whole screen as text, lines joined by `\n`.
    pub fn render(&self) -> String {
        (1..=self.rows)
            .map(|l| self.line_str(l))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dec(neg: bool, ds: &[u8], sc: i16) -> Decimal {
        Decimal { negative: neg, digits: ds.to_vec(), scale: sc }
    }

    #[test]
    fn places_text_at_one_based_line_col() {
        let mut s = Screen::new(2, 10);
        s.put(1, 1, b"HELLO");
        s.put(2, 4, b"XY");
        assert_eq!(s.line_str(1), "HELLO     ");
        assert_eq!(s.line_str(2), "   XY     ");
    }

    #[test]
    fn clips_at_the_right_edge_and_ignores_bad_lines() {
        let mut s = Screen::new(1, 5);
        s.put(1, 3, b"ABCDE"); // only "ABC" fits from col 3
        assert_eq!(s.line_str(1), "  ABC");
        s.put(9, 1, b"NOPE"); // out-of-range line: no-op
        assert_eq!(s.line_str(1), "  ABC");
    }

    #[test]
    fn renders_numeric_field_through_the_16c_court() {
        let mut s = Screen::new(1, 12);
        // -12.5 in PIC $$,$$9.99CR -> the cobc-faithful "   $12.50CR" (GNURUST.16C).
        s.put_edited(1, 1, &dec(true, &[1, 2, 5], 1), "$$,$$9.99CR").unwrap();
        assert_eq!(s.line_str(1).trim_end(), "   $12.50CR");
    }
}
