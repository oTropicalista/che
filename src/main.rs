#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;

use editor::Editor;
pub use  terminal::Terminal;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    Editor::default().run();
}

