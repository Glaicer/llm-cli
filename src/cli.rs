use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "llm", version, about = "Quick AI answers from the terminal")]
pub struct Args {
    /// Single (non-interactive) request: print the answer and exit.
    #[arg(short = 's', long = "single")]
    pub single: bool,

    /// The prompt. Seeds the conversation in interactive mode; with --single it is the one request.
    #[arg(value_name = "PROMPT")]
    pub prompt: Option<String>,
}
