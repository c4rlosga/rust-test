use eyre::Result;
use std::env;

fn main() -> Result<()> {
    init()?;

    let prompt = env::var("PROMPT").unwrap();

    let mut rl = rustyline::Editor::<()>::new();
    while let Ok(line) = rl.readline(&prompt) {
        match line.as_str().trim() {
            "exit" => break,
            line => println!("We got {}", line),
        }
    }

    Ok(())
}

fn init() -> Result<()> {
    dotenv::dotenv()?;
    color_eyre::install()?;

    Ok(())
}
