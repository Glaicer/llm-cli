mod chat;
mod cli;
mod client;
mod config;
mod repl;

use chat::Conversation;
use clap::Parser;
use cli::Args;
use client::ReqwestClient;
use config::LoadOutcome;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let cfg = match config::load()? {
        LoadOutcome::Created => return Ok(()),
        LoadOutcome::Loaded(c) => c,
    };
    let client = ReqwestClient::new(&cfg);

    if args.single {
        let prompt = args.prompt.ok_or_else(|| -> Box<dyn std::error::Error> {
            "single mode requires a prompt".into()
        })?;
        let mut conv = Conversation::new(&cfg.system_instruction);
        let content = repl::turn(&mut conv, &client, &prompt)?;
        println!("{content}");
    } else {
        let conv = Conversation::new(&cfg.system_instruction);
        repl::run_repl(conv, &client, args.prompt)?;
    }
    Ok(())
}
