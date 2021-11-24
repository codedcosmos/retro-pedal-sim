use std::io::Stdout;

pub mod compressor;
pub mod distortion;
pub mod eq;
pub mod preamp;
pub mod reverb;

pub trait Pedal {
	// Draws the pedal at it's current state
	// At the position, returns an increment based on it's height
	fn draw(&self, write: &mut Stdout,  x: u16, y: u16) -> u16;
}