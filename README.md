# gnucobol-rs-tui

A COBOL **Screen Section** primitive: a 1-based `LINE`/`COL` terminal cell matrix that places `DISPLAY`
fields, and renders numeric fields through the **oracle-proven**
[`gnucobol-rs`](https://github.com/infinityabundance/gnucobol-rs) edited-encode court (`GNURUST.16C`) — so a
`PIC $$,$$9.99CR` field on screen carries the exact cobc-faithful presentation bytes.

```rust
use gnucobol_rs_tui::Screen;
use gnucobol_rs::Decimal;

let mut s = Screen::new(2, 12);
s.put(1, 1, b"BALANCE:");
s.put_edited(2, 1, &Decimal { negative: true, digits: vec![1,2,5], scale: 1 }, "$$,$$9.99CR").unwrap();
assert_eq!(s.line_str(2).trim_end(), "   $12.50CR");
```

Screen *positioning* is a from-scratch primitive (cobc's `screenio.c` is not yet a sealed court); the
numeric/edited *content* it places is court-backed via `GNURUST.16C`.

## License
LGPL-3.0-or-later — a faithful derivative of GnuCOBOL/libcob (FSF copyright retained). See COPYING.LESSER (+ COPYING).
