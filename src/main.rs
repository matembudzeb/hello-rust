use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main(){
	let stdout = stdout();
	let message = String::from("Hello World! I am Bornwell");
	let width = message.chars().count();
	let mut writer = BufWriter::new(stdout.lock());
	println!("Seems the macro output below is printed before the say function despite being the last line");
	say(&message, width, &mut writer).unwrap();
	println!("I am a Rustacean now. Given this new identity by the println! macro. Watch the extra ! otherwise ferris will bit yah");
}
