use std::str::Chars;
use std::iter::Peekabe;

pub struct Tokenizer<'a> {
    // Attach the lifetime to Peekabe<Chars> reference
    // to attach the reference to the struct's lifetime
    expr: Peekabe<Chars<'a>>;
}
