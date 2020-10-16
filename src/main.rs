#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod document;
mod terminal;
mod row;

pub use row::Row;
pub use editor::Position;
pub use document::Document;
pub use terminal::Terminal;

use editor::Editor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    Editor::default().run();
}

