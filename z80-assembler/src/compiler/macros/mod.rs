use crate::compiler::instructions::CompileError;
use crate::compiler::r#macro::Macro;
use crate::parser::tokenizer::{BufferedTokenizer, Tokenizer};
use crate::parser::{Token, TokenValue};
use std::collections::HashMap;

pub fn compile_macro(
    cmd: String,
    tokens: Vec<Token>,
    tokenizer: &mut BufferedTokenizer,
    macros: &mut HashMap<String, Macro>,
) -> Result<(), CompileError> {
    match cmd.as_str() {
        "#defm" => {
            let (name, args) = get_macro_name_and_args(tokens)?;

            let mut m = Macro {
                name: name.to_string(),
                args,
                tokens: vec![],
            };
            loop {
                let t = tokenizer.next()?;
                if let TokenValue::Directive(d) = &t.token {
                    if d.starts_with("#endm") {
                        macros.insert((&m.name).to_string(), m);
                        return Ok(());
                    }
                }

                if let TokenValue::EOF = &t.token {
                    panic!("unexpected EOF");
                }

                m.tokens.push(t);
            }
        }
        "#exec" => {
            let (name, args) = get_macro_name_and_args(tokens)?;

            if let Some(m) = macros.get(name.as_str()) {
                let mut out = vec![];

                for t in &m.tokens {
                    if let TokenValue::Identifier(ident) = &t.token {
                        if let Some(i) = m.args.iter().position(|x| {
                            if let Some(TokenValue::Identifier(x1)) = x.first().map(|x| &x.token) {
                                x1 == ident
                            } else {
                                false
                            }
                        }) {
                            for a in args[i].iter() {
                                out.push(a.clone());
                            }
                        } else {
                            out.push(t.clone())
                        }
                    } else {
                        out.push(t.clone())
                    }
                }

                tokenizer.push_front(&out)
            } else {
                panic!("macro: '{:?}' not found", args)
            }
        }
        _ => unimplemented!("unhandled macro: '{}'", cmd),
    }
    Ok(())
}

fn get_macro_name_and_args<'a>(
    tokens: Vec<Token>,
) -> Result<(String, Vec<Vec<Token>>), CompileError> {
    let name = if let Some(Token {
        token: TokenValue::Identifier(name),
        ..
    }) = tokens.first()
    {
        name
    } else {
        unimplemented!("expected macro name error");
    };

    let mut args = vec![];
    let mut arg_tokens = tokens.iter().skip(1);

    let mut a = vec![];
    loop {
        match arg_tokens.next() {
            Some(Token {
                token: TokenValue::Comma,
                ..
            }) => {
                args.push(a);
                a = vec![];
            }
            Some(t) => a.push(t.clone()),
            None => {
                args.push(a);
                break;
            }
        }
    }

    Ok((name.clone(), args))
}
