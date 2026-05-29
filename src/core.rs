
#[derive(Debug)]
pub enum TokenType{
	KEYWORD(String),
	IDENTIFIER(String),
	LEFTPAR,
	RIGHTPAR,
	EQ,
	NUMBER(String),	
}

#[derive(Debug)]
pub struct Text {
	pub source: String,
	pub current: usize,
	pub length: usize,
}
