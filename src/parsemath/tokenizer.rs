use std::str::Chars;
use std::iter::Peekabe;

pub struct Tokenizer<'a> {
    expr: Peekabe<Chars<'a>>;
}
