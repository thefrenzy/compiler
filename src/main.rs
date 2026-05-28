use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;


enum TokenType{
	
	IDENTIFIER(String),
	LEFTPAR,
	RIGHTPAR,
	EQ,
	Number(i64),
	
}


struct Text {
	source: String,
	current: u64,
}


fn scanner(text: &Text, tokens: &mut Vec<TokenType>){
	

}

fn main() -> std::io::Result<()>{
	let file = File::open("foo.txt")?;
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;


	if contents.is_empty(){
		return Ok(());
	}
		
	let text = 
			Text{
			source : contents,
			current : 0,
			};
	
	let mut tokens = vec![];
	
	scanner(&text, &mut tokens);
	
	Ok(())
	
}