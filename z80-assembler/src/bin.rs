use z80_assembler::{Compiler, InMemorySourceProvider, SourceHeader};

use std::env;

fn help() {
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        3 => {
            let source = &args[1];
            let dest = &args[2];

            let s = std::fs::read_to_string(source).unwrap();
            let res = Compiler::new(
                InMemorySourceProvider {
                    files: vec![(
                        SourceHeader {
                            filename: source.to_string(),
                        },
                        s,
                    )],
                },
                64 * 1024,
            )
            .compile()
            .unwrap();

            std::fs::write(dest, res).unwrap();
        }
        _ => {
            help();
        }
    }
}
