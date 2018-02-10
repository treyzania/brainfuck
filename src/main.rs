extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;
use std::fs::File;
use std::io::{Read, Write};

// The StructOpt crate is really easy to use for quick argument parsing.
#[derive(StructOpt, Debug)]
#[structopt(name = "brainfuck", about = "Brainfuck interpreter and compiler.")]
struct Options {
    #[structopt(name = "input file", help = "A Brainfuck code file to compile.")]
    pub input_file: String,
    #[structopt(short = "o", long = "out",
                help = "A Rust source file to write the compiled form to. (WARNING: overwrites)")]
    pub out_file: Option<String>,
    #[structopt(short = "s", long = "tape-size", default_value = "20000",
                help = "Size of the tape. (4 bytes on 32-bit, 8 bytes on 64-bit)")]
    pub tape_size: usize,
}

fn main() {
    let options = Options::from_args();

    // Firstly, read all the text from the input file they passed
    // when executing the program, and place it into a new String.
    let mut source = String::new();
    {
        let mut file = File::open(options.input_file).unwrap();
        file.read_to_string(&mut source).unwrap();
    }

    // Tokenize the Brainfuck source file.
    let tokens = tokenize(&source);
    // Compile using the tokens gathered from the file.
    compile(&tokens, options.out_file, options.tape_size);
}

#[derive(PartialEq, Copy, Clone)]
enum Token {
    Add,       // +
    Sub,       // -
    Right,     // >
    Left,      // <
    Read,      // ,
    Write,     // .
    BeginLoop, // [
    EndLoop,   // ]
}
use self::Token::*;

/// Splits up the Brainfuck source file into tokens.
/// Tokens are easier to parse because they are flat
/// streams, without whitespace or punctuation you get
/// when you compile directly from a source file.
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::<Token>::new();

    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => tokens.push(Add),
            '-' => tokens.push(Sub),
            '>' => tokens.push(Right),
            '<' => tokens.push(Left),
            ',' => tokens.push(Read),
            '.' => tokens.push(Write),
            '[' => tokens.push(BeginLoop),
            ']' => tokens.push(EndLoop),
            _ => {}
        }
    }

    tokens
}

const PREFACE_FORMER: &'static str = "#![allow(unused_imports)]
#![allow(dead_code)]

use std::io::{stdout, stdin, Read, Write};

";

const PREFACE_LATTER: &'static str = "fn ptr_wrap_add(ptr: &mut usize) {
    if *ptr == TAPE_SIZE - 1 {
        *ptr = 0;
    } else {
        *ptr += 1;
    }
}

fn ptr_wrap_sub(ptr: &mut usize) {
    if *ptr == 0 {
        *ptr = TAPE_SIZE - 1;
    } else {
        *ptr -= 1;
    }
}

fn main() {
    let mut tape = [0u8; TAPE_SIZE];
    let mut ptr = 0usize;

    let stdout = stdout();
    let mut handle = stdout.lock();

";

static SOURCE_END: &'static str = "}
";

fn compile(tokens: &[Token], out_file: Option<String>, tape_size: usize) {
    // Insert the preface, which in this instance, is just boiler-plate code
    // that would need to be created for every Brainfuck program, regardless
    // of what it does.
    let mut output_source = String::from(PREFACE_FORMER);
    output_source.push_str(format!("const TAPE_SIZE: usize = {};\n\n", tape_size).as_str());
    output_source.push_str(PREFACE_LATTER);

    // Tabs are just for pretty printing indented text.
    let mut indentation = 1u32;

    fn indent(count: u32) -> String {
        let mut indentation = String::new();
        for _ in 0..count {
            indentation.push_str("    ");
        }
        indentation
    }

    // Now we can just iterate over every one of our tokens.
    for &token in tokens {
        match token {
            Add => {
                // Here, I just add the indention before the line,
                // to make the output look cleaner and more hand-written.
                output_source.push_str(&indent(indentation));
                output_source.push_str("tape[ptr] = tape[ptr].wrapping_add(1);\n");
            }
            Sub => {
                output_source.push_str(&indent(indentation));
                output_source.push_str("tape[ptr] = tape[ptr].wrapping_sub(1);\n");
            }
            Right => {
                output_source.push_str(&indent(indentation));
                output_source.push_str("ptr_wrap_add(&mut ptr);\n");
            }
            Left => {
                output_source.push_str(&indent(indentation));
                output_source.push_str("ptr_wrap_sub(&mut ptr);\n");
            }
            Read => {
                // Here, I just made a variable containing the indentation so that I
                // can just reference it.
                let mut tab = &indent(indentation);

                output_source.push_str(format!(
                    "{}match stdin().bytes().next() {{\n{}    Some(byte) => tape[ptr] = byte.unwrap(),\n{}    None => tape[ptr] = 0,\n{}}}",
                    tab, tab, tab, tab)
                .as_str());
            }
            Write => {
                output_source.push_str(&indent(indentation));
                output_source.push_str("handle.write(&[tape[ptr]]).unwrap();\n");
            }
            BeginLoop => {
                output_source.push_str(&indent(indentation));
                output_source.push_str("while tape[ptr] != 0 {\n");
                indentation += 1;
            }
            EndLoop => {
                output_source.push_str(&indent(indentation));
                output_source.push_str("}\n");
                indentation -= 1;
            }
        }
    }

    // Complete the boiler-plate code by adding the missing closing brace.
    output_source.push_str(SOURCE_END);

    // Finally, either write the compiled program to a file the user passed
    // as arguments, or write it to the standard output stream.
    match out_file {
        Some(file_name) => {
            let mut file = File::create(file_name.as_str()).unwrap();
            file.write_all(output_source.as_bytes()).unwrap();
        }
        None => println!("{}", output_source),
    }
}
