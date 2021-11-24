mod pedal;

use std::io::{stderr, stdout, Write};
use crate::pedal::preamp::PreampPedal;

use crossterm::{
	cursor::{Hide, MoveTo, Show},
	event,
	event::{Event, KeyCode, KeyEvent},
	execute, queue,
	Result,
	style::Print,
	terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::pedal::compressor::CompressorPedal;
use crate::pedal::distortion::DistortionPedal;
use crate::pedal::eq::EqPedal;
use crate::pedal::Pedal;
use crate::pedal::reverb::ReverbPedal;

pub fn read_char() -> char {
	loop {
		if let Event::Key(KeyEvent {
			code: KeyCode::Char(c),
			..
		}) = event::read().expect("Failed to read event")
		{
			return c;
		}
	}
}

pub fn retro() {
	let mut write = stdout();

	// Enter an alternative screen
	queue!(
		write,
		EnterAlternateScreen, // enter alternate screen
		Hide                  // hide the cursor
	).expect("Failed to enter clear mode");

	// Enter raw mode
	terminal::enable_raw_mode().expect("Failed enable raw mode");

	// Create pedals
	let mut pedals: Vec<Box<dyn Pedal>> = Vec::new();

	pedals.push(Box::from(PreampPedal {

	}));
	pedals.push(Box::from(ReverbPedal {

	}));
	pedals.push(Box::from(CompressorPedal {

	}));
	pedals.push(Box::from(DistortionPedal {

	}));
	pedals.push(Box::from(EqPedal {

	}));

	loop {
		let mut y = 1;
		for pedal in &pedals {
			y += pedal.draw(&mut write, 1, y);
		}

		write.flush().expect("Failed to flush buffer");

		let user_char = read_char(); // we wait for the user to hit a key

		if user_char == 'q' {
			break
		}
	}

	// Leave alternative screen
	execute!(write, Show, LeaveAlternateScreen).expect("Failed leave alternative screen");

	// Exit raw mode
	terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

fn main() {
   retro()
}
