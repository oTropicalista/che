#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

pub use  terminal::Terminal;
pub use editor::Position;

use editor::Editor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    Editor::default().run();
}

