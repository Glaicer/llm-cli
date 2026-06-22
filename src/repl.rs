use crate::chat::Conversation;
use crate::client::{ChatClient, ClientError};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

enum ReplCommand {
    Exit,
    Skip,
    Send(String),
}

fn parse_repl_command(line: &str) -> ReplCommand {
    match line.trim() {
        "" => ReplCommand::Skip,
        "exit" | "quit" => ReplCommand::Exit,
        trimmed => ReplCommand::Send(trimmed.to_string()),
    }
}

pub fn turn<C: ChatClient>(
    conv: &mut Conversation,
    client: &C,
    input: &str,
) -> Result<String, ClientError> {
    conv.add_user(input);
    match client.chat(conv.messages()) {
        Ok(content) => {
            conv.add_assistant(&content);
            Ok(content)
        }
        Err(e) => {
            conv.pop_user();
            Err(e)
        }
    }
}

pub fn run_repl<C: ChatClient>(
    mut conv: Conversation,
    client: &C,
    initial: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    if let Some(first) = initial {
        match turn(&mut conv, client, &first) {
            Ok(content) => println!("{content}"),
            Err(e) => eprintln!("error: {e}"),
        }
    }
    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                match parse_repl_command(&line) {
                    ReplCommand::Exit => break,
                    ReplCommand::Skip => continue,
                    ReplCommand::Send(text) => match turn(&mut conv, client, &text) {
                        Ok(content) => println!("{content}"),
                        Err(e) => eprintln!("error: {e}"),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!();
                break;
            }
            Err(ReadlineError::Eof) => break,
            Err(e) => {
                eprintln!("error: {e}");
                break;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::{Conversation, Message};
    use crate::client::{ChatClient, ClientError};

    struct MockOk(&'static str);
    impl ChatClient for MockOk {
        fn chat(&self, _m: &[Message]) -> Result<String, ClientError> {
            Ok(self.0.to_string())
        }
    }
    struct MockErr;
    impl ChatClient for MockErr {
        fn chat(&self, _m: &[Message]) -> Result<String, ClientError> {
            Err(ClientError::Status(403, "forbidden".into()))
        }
    }

    #[test]
    fn turn_appends_user_and_assistant_on_success() {
        let mut c = Conversation::new("sys");
        let content = turn(&mut c, &MockOk("hi"), "hello").unwrap();
        assert_eq!(content, "hi");
        assert_eq!(c.messages().len(), 3);
    }

    #[test]
    fn turn_pops_user_on_error() {
        let mut c = Conversation::new("sys");
        let res = turn(&mut c, &MockErr, "hello");
        assert!(res.is_err());
        assert_eq!(c.messages().len(), 1);
    }

    #[test]
    fn parse_repl_command_classifies() {
        assert!(matches!(parse_repl_command("exit"), ReplCommand::Exit));
        assert!(matches!(parse_repl_command(" quit "), ReplCommand::Exit));
        assert!(matches!(parse_repl_command(""), ReplCommand::Skip));
        assert!(matches!(parse_repl_command("   "), ReplCommand::Skip));
        match parse_repl_command(" list files ") {
            ReplCommand::Send(s) => assert_eq!(s, "list files"),
            _ => panic!("expected Send"),
        }
    }
}
