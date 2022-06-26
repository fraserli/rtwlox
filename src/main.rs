mod lexer;

use lexer::lex;

use anyhow::{Context, Result};

use std::fs::File;
use std::io::{Read, Write};

fn repl() -> Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush()?;

        let mut line = String::new();

        if std::io::stdin().read_line(&mut line)? == 0 {
            break Ok(());
        } else {
            let tokens = lex(&line)?;

            dbg!(tokens);
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if let Some(path) = args.get(1) {
        let mut file = File::open(path).with_context(|| format!("failed to open {path}"))?;
        let mut source = String::new();
        file.read_to_string(&mut source)?;
        let tokens = lex(&source)?;

        dbg!(tokens);
    } else {
        repl()?;
    };

    Ok(())
}
