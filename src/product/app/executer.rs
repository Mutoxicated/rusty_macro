use std::{fmt::Display, fs::File, io::{Read, Write}, path::PathBuf};

use lexer::{token::{Token, TokenEnum}, Lexer};

use crate::csmacro::call::MacroCall;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    MacroStart,
    MacroEnd
}

impl TokenEnum<String> for TokenType {
    fn out(lexer:&mut Lexer<Self, String>) -> bool 
        where Self: Sized {

        let comment_length = lexer.state.len();
        let str = lexer.peek_str(comment_length);
        if str.is_none() {
            return false
        }
        let str = str.unwrap();

        let comment = lexer.state.clone();
        if str == comment {
            let string = lexer.consume_str(comment_length);
            if string.is_none() {
                return false
            }
            let string = string.unwrap();
            lexer.push_str(string.as_str());
            lexer.try_lexy();
            let buf = lexer.read_buffer();
            if buf == comment.clone()+"MACRO_START" {
                lexer.add_token(TokenType::MacroStart, buf.as_str());
                return true
            }else if buf == comment+"MACRO_END" {
                lexer.add_token(TokenType::MacroEnd, buf.as_str());
                return true
            }
        }

        false
    }
}

pub struct Executer {
    path: PathBuf,
    string: String,
    index:usize,
    tokens:Vec<Token<TokenType, String>>,
    tabs:String
}

pub enum ExecutionError {
    NoMacroStart,
    NoMacroEnd,
    EmptyMacroList,
    NoFile
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            match self {
            ExecutionError::NoMacroStart => "No macro start flag was placed in the script. Please press the 'Info' button next to the script path.",
            ExecutionError::NoMacroEnd => "No macro end flag was placed in the script. Please press the 'Info' button next to the script path.",
            ExecutionError::EmptyMacroList => "No macro calls existed in the list to execute.",
            ExecutionError::NoFile => "No file was given to execute the macros on."
            }
        )?;

        Ok(())
    }
}

impl Executer {
    pub fn new(path:PathBuf, comment:&str) -> Self {
        let mut buf:String = String::new();
        let mut file = File::options()
            .read(true)
            .open(&path)
            .unwrap();
        let _ = file.read_to_string(&mut buf);
        let mut lexer:Lexer<TokenType, String> = Lexer::with_state(buf.chars().collect(), comment.to_owned());
        Self { 
            path, 
            string:buf, 
            index: 0, 
            tokens: lexer.action(),
            tabs: String::new()
        }
    }

    pub fn action(&mut self, macro_calls:&[MacroCall]) -> Result<Option<()>, ExecutionError> {
        if macro_calls.is_empty() {
            return Err(ExecutionError::EmptyMacroList)
        }

        let mut macro_start: Option<Token<TokenType, String>> = None;
        let mut macro_end: Option<Token<TokenType, String>> = None;

        for token in self.tokens.clone() {
            if token.r#type() == TokenType::MacroStart {
                macro_start = Some(token);
            }else if token.r#type() == TokenType::MacroEnd {
                macro_end = Some(token);
            }
        }

        println!("[Debug] tokens: {:?}", &self.tokens);

        if macro_start.is_none() {
            return Err(ExecutionError::NoMacroStart)
        }
        if macro_end.is_none() {
            return Err(ExecutionError::NoMacroEnd)
        }
        let macro_end = macro_end.unwrap();
        let macro_start = macro_start.unwrap();

        self.index = macro_start.index().1;
        
        self.get_identation(macro_start.index().0-1);

        for i in (self.index..macro_end.index().0).rev() {
            self.string.remove(i);
        }

        self.insert('\n');
        self.insert('\n');
        self.insert_str(self.tabs.clone().as_str());
        self.index -= self.tabs.len()+1;
        let call_length = macro_calls.len();
        for i in 0..call_length {
            self.insert_str(macro_calls[i].expand(&self.tabs).as_str());
            if i != call_length-1 {
                self.insert('\n');
            }
        }

        let mut file = File::options()
            .read(false)
            .write(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();

        let _ = file.write_all(self.string.as_bytes());

        Ok(Some(()))
    }

    fn get_identation(&mut self, index:usize) {
        let chars:Vec<char> = self.string.chars().collect();
        let mut tabs = String::new();
        let mut index = index;
        while chars[index] == ' ' {
            index -= 1;
            tabs += " ";
        }

        self.tabs = tabs;
    }

    fn insert_str(&mut self, str:&str) {
        let len = str.len();
        self.string.insert_str(self.index, str);
        self.index += len;
    }

    fn insert(&mut self, c:char) {
        self.string.insert(self.index, c);
        self.index += 1;
    }
}