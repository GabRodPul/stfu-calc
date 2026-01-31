use stfu_calc::*;

use fltk::{
    app::{Sender, event_key}, button::Button, enums::{Event, Key}, group::GridRange, prelude::*, window::Window, *
};
use fltk_grid::Grid;

fn btn_init(
    grid: &mut Grid,
    row: impl Into<GridRange>,
    col: impl Into<GridRange>,
    s: Sender<CalcKey>,
    key: CalcKey,
) {
    let mut btn = Button::default().with_label(&key.to_string());
    btn.emit(s, key);
    grid.set_widget(&mut btn, row, col).unwrap();
}

fn main() {
    const ROWS: i32 = 5;
    const COLS: i32 = 5;
    let app = app::App::default();
    let mut win = Window::default().with_size(400, 600).with_label("STFU Calculator (fltk)");
    let mut grid = Grid::default_fill();
    let (s, r) = app::channel::<CalcKey>();

    grid.show_grid(false);
    grid.set_layout(ROWS, COLS);

    let display = &mut frame::Frame::default().with_label("0");
    grid.set_widget(display, 0, 0..5).unwrap();

    for (i, r) in CalcKey::ROWS.iter().enumerate() {
        r.into_iter()
            .enumerate()
            .for_each(|(j, k)| btn_init(&mut grid, i+1, j, s, *k))
    }

    btn_init(&mut grid, 4, 2..5, s, CalcKey::Eq);

    grid.end();
    win.end();
    win.show();

    let mut buf = CalcBuf::new(display.label());

    while app.wait() {
        win.handle(move |_, event| event == Event::KeyDown && match event_key() {
            Key::BackSpace  => { s.send(CalcKey::CEnt); true },
            Key::Escape     => { s.send(CalcKey::CAll); true }
            Key::Enter      => { s.send(CalcKey::Eq);   true },
            
            k @ _ => {
                if let Ok(k) = CalcKey::try_from(k.bits() as u8 as char) {
                    s.send(k);
                    true
                } else {
                    false
                }
            }
        }); 

        if let Some(msg) = r.recv() {
            buf.push(msg);
            display.set_label(&buf.0);
        }
    }
}
