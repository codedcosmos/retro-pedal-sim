use std::io::{Stdout, stdout};
use crossterm::{
	cursor::{Hide, MoveTo, Show},
	event,
	event::{Event, KeyCode, KeyEvent},
	execute, queue,
	Result,
	style::Print,
	terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::Pedal;

pub struct EqPedal {

}

impl Pedal for EqPedal {
	fn draw(&self, mut write: &mut Stdout, x: u16, y: u16) -> u16 {
		queue!(
			*write,
			MoveTo(x, y+0), Print("/======================================\\"),
			MoveTo(x, y+1), Print("|   EQ                                 |"),
			MoveTo(x, y+2), Print("|                                      |"),
			MoveTo(x, y+3), Print("|                                      |"),
			MoveTo(x, y+4), Print("|                                      |"),
			MoveTo(x, y+5), Print("|                                      |"),
			MoveTo(x, y+6), Print("|                                      |"),
			MoveTo(x, y+7), Print("|                                      |"),
			MoveTo(x, y+8), Print("\\======================================/"),
		).expect("Failed to write line");

		// Return number of lines
		9
	}
}