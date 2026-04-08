//! ─── KOMUT PARSER ───
//!
//! Komut ayristirma ve donusturme

use std::collections::HashMap;

/// Parse edilmis komut
#[derive(Debug, Clone)]
pub struct ParsedCommand {
    /// Komut adi
    pub command: String,
    /// Alt komut
    pub subcommand: Option<String>,
    /// Argumanlar
    pub args: Vec<String>,
    /// Secenekler (--flag value)
    pub options: HashMap<String, String>,
    /// Bayraklar (--flag)
    pub flags: Vec<String>,
    /// Ham girdi
    pub raw: String,
}

/// Komut parser
pub struct CommandParser;

impl CommandParser {
    pub fn new() -> Self {
        Self
    }

    /// Komutu ayristir
    pub fn parse(&self, input: &str) -> ParsedCommand {
        let trimmed = input.trim();
        let raw = trimmed.to_string();

        if trimmed.is_empty() {
            return ParsedCommand {
                command: String::new(),
                subcommand: None,
                args: Vec::new(),
                options: HashMap::new(),
                flags: Vec::new(),
                raw,
            };
        }

        let tokens = self.tokenize(trimmed);
        let (command, tokens) = self.extract_command(&tokens);
        let (subcommand, tokens) = self.extract_subcommand(&tokens);
        let (args, options, flags) = self.parse_tokens(tokens);

        ParsedCommand {
            command,
            subcommand,
            args,
            options,
            flags,
            raw,
        }
    }

    /// Token'lara ayir
    fn tokenize(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut quote_char = ' ';

        for ch in input.chars() {
            match ch {
                '"' | '\'' => {
                    if !in_quotes {
                        in_quotes = true;
                        quote_char = ch;
                    } else if ch == quote_char {
                        in_quotes = false;
                    } else {
                        current.push(ch);
                    }
                }
                ' ' | '\t' => {
                    if in_quotes {
                        current.push(ch);
                    } else if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    /// Ilk token'i komut olarak al
    fn extract_command(&self, tokens: &[String]) -> (String, Vec<String>) {
        if tokens.is_empty() {
            (String::new(), Vec::new())
        } else {
            (tokens[0].to_lowercase(), tokens[1..].to_vec())
        }
    }

    /// Ikinci token'i alt komut olarak al
    fn extract_subcommand(&self, tokens: &[String]) -> (Option<String>, Vec<String>) {
        if tokens.is_empty() {
            (None, Vec::new())
        } else if tokens[0].starts_with('-') {
            (None, tokens.to_vec())
        } else {
            (Some(tokens[0].to_lowercase()), tokens[1..].to_vec())
        }
    }

    /// Token'lari arguman, secenek ve bayrak olarak ayir
    fn parse_tokens(&self, tokens: Vec<String>) -> (Vec<String>, HashMap<String, String>, Vec<String>) {
        let mut args = Vec::new();
        let mut options = HashMap::new();
        let mut flags = Vec::new();

        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];

            if token.starts_with("--") {
                // Uzun secenek: --name value veya --flag
                let name = token[2..].to_string();
                
                // Bir sonraki token deger mi?
                if i + 1 < tokens.len() && !tokens[i + 1].starts_with('-') {
                    options.insert(name, tokens[i + 1].clone());
                    i += 2;
                } else {
                    flags.push(name);
                    i += 1;
                }
            } else if token.starts_with('-') && token.len() > 1 {
                // Kisa secenek: -n value veya -f
                let name = token[1..].to_string();
                
                if i + 1 < tokens.len() && !tokens[i + 1].starts_with('-') {
                    options.insert(name, tokens[i + 1].clone());
                    i += 2;
                } else {
                    flags.push(name);
                    i += 1;
                }
            } else {
                // Normal arguman
                args.push(token.clone());
                i += 1;
            }
        }

        (args, options, flags)
    }

    /// Argumanlarin sonunu birlestir (bosluklu)
    pub fn join_args(&self, args: &[String]) -> String {
        args.join(" ")
    }
}

impl Default for CommandParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parse() {
        let parser = CommandParser::new();
        let cmd = parser.parse("status");
        
        assert_eq!(cmd.command, "status");
        assert!(cmd.subcommand.is_none());
    }

    #[test]
    fn test_subcommand_parse() {
        let parser = CommandParser::new();
        let cmd = parser.parse("memory list");
        
        assert_eq!(cmd.command, "memory");
        assert_eq!(cmd.subcommand, Some("list".to_string()));
    }

    #[test]
    fn test_option_parse() {
        let parser = CommandParser::new();
        let cmd = parser.parse("agent spawn --model qwen/test");
        
        assert_eq!(cmd.command, "agent");
        assert_eq!(cmd.options.get("model"), Some(&"qwen/test".to_string()));
    }

    #[test]
    fn test_flag_parse() {
        let parser = CommandParser::new();
        let cmd = parser.parse("task list --verbose --all");
        
        assert!(cmd.flags.contains(&"verbose".to_string()));
        assert!(cmd.flags.contains(&"all".to_string()));
    }

    #[test]
    fn test_quoted_parse() {
        let parser = CommandParser::new();
        let cmd = parser.parse("task add \"Python script yaz\"");
        
        assert_eq!(cmd.args[0], "Python script yaz");
    }
}
