use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod core;
mod lexer;

use crate::core::Text;

use lexer::scanner;



fn main() -> std::io::Result<()>{
	let file = File::open("source.txt")?;
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;


	if contents.is_empty(){
		return Ok(());
	}
		
	let mut text = 
			Text{
			source : contents,
			current : 0,
			length : 0,
			};
	text.length = text.source.len();
	
	let mut tokens = vec![];
	
	scanner(&mut text, &mut tokens);
	
	println!("{:?}", tokens);
	
	Ok(())
	
}