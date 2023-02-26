# Expression Parser

## Technical Requirements

The tool should accept an arithmetic expression as input, evaluate it, and provide the numerical output as a floating-point number. For example, the expression `1+2*3.2+(4/2-3/2)-2.11+2^4` should evaluate to `21.79`.

The arithmetic operations in scope are addition (+), subtraction (-), multiplication (*), division (/), power (^), the negative prefix (-), and expressions enclosed in parentheses ().

## Challenges

Challenges to be solved are -
- User must be able to input the expression as a free string
- Number, arith operators and parenthesis should be processed
- Rules of opertator precedence must be taken into account
- User must not give spaces between the expression but the program must be able to handle it
- When encountered the decimal point, continue reading the number to next operator
- Invalid inputs must be dealt with and appropriate error messages mush be shown.
	- Since we do not deal with variables in the program `2+a` must error out as invalid input
	- Incomplete OR mismatch of parenthesis much error out
	- If the operator is not recognizable, program must error out

## Design of the system

![Pasted image 20230225153826.png](./Pasted%20image%2020230225153826.png)

- Lexer(Tokenizer) will convert the entire expression into tokens i.e. either the numbers or operators or parenthesis.
- Parser will convert the tokens into AST (Abstract Syntax Tree). AST will handle the evaluation priority ex: addition < multiplication < parenthesis.
- Evaluator will evaluate the AST in proper order of priority.

### Lexers, Parsers and ASTs
Lexers and Parsers are the concepts used in computer science to build interpreters and compilers.

A Lexers (also Tokenizer) splits the text (source code) into words and assign a lexical meaning to it such as keyboard, expression, operator, function call etc. Lexers generate tokens and hence also know as Tokenizers.

A parser take the output of the lexer and arrange them into a tree structure called AST (Abstract Syntax Tree). With AST the compiler can generate a machine code and an interpreter can evaluate an instruction.

## Coding the Solution

### Code Structure

```
❯ tree .
.
├── Cargo.toml
├── readme.md
└── src
    ├── main.rs
    └── parsemath
        └── mod.rs
```

In rust - a `mod.rs` file inside a dir makes it a module. `mod.rs` can export the modules from the source dir example as follows - 
```rs
pub mod ast;
pub mod parser;
pub mod token;
pub mod tokenizer;
```

### Building the Tokenizer

Purpose of Tokenizer -
- store user input expression in a local variable
- represent the expression into output tokens

Example of a sample expression input expression - `1+21*3.2`. These will be stored as input str.

Tokens generated from above will be => Num(1.0), Add, Num(21.0), Multiply, Num(3.2).

Now in order to properly parse the expression char-by-char, we also need to look ahead...
	ex: to tokenize `21` into Num(21), we need to read char `2` followed by char `1`, followed by `*`.

To implement this we need a `peekable iterator`. 
	example for `1+2`
	itr.next() will return `1`
	itr.peak() will return `+` but not consume `+` and iterator would still be at position `1`


