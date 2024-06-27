use std::{fmt::Display, fs::File, io::{Read, Write}, path::PathBuf};

use lexer::{token::{Token, TokenEnum, TokenType}, Lexer};

use crate::csmacro::call::MacroCall;

lexer::custom_token_enum!(
    CustomTokenType;
    MacroStart,
    MacroEnd
);

impl TokenEnum for CustomTokenType {
    fn special(lexer:&mut Lexer<Self>) -> bool 
        where Self: Sized {

        use TokenType::*;
        
        let str = lexer.peek_str(2);
        if str.is_none() {
            return false
        }
        let str = str.unwrap();
        if str == "//" {
            let string = lexer.consume_str(2);
            if string.is_none() {
                return false
            }
            let string = string.unwrap();
            lexer.push_str(string.as_str());
            lexer.try_lexy();
            let buf = lexer.read_buffer();
            if buf == "//MACRO_START" {
                lexer.add_token(Custom(CustomTokenType::MacroStart), buf.as_str());
                return true
            }else if buf == "//MACRO_END" {
                lexer.add_token(Custom(CustomTokenType::MacroEnd), buf.as_str());
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
    tokens:Vec<Token<CustomTokenType>>,
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
    pub fn new(path:PathBuf) -> Self {
        let mut buf:String = String::new();
        let mut file = File::options()
            .read(true)
            .open(&path)
            .unwrap();
        let _ = file.read_to_string(&mut buf);
        let mut lexer:Lexer<CustomTokenType> = Lexer::new(buf.chars().collect());
        Self { 
            path, 
            string:buf, 
            index: 0, 
            tokens: lexer.action(),
            tabs: String::new()
        }
    }

    pub fn action(&mut self, macro_calls:&Vec<MacroCall>) -> Result<(), ExecutionError> {
        if macro_calls.is_empty() {
            return Err(ExecutionError::EmptyMacroList)
        }

        let mut macro_start: Option<Token<CustomTokenType>> = None;
        let mut macro_end: Option<Token<CustomTokenType>> = None;

        for token in self.tokens.clone() {
            if let TokenType::Custom(x) = token.r#type() {
                if x == CustomTokenType::MacroStart {
                    macro_start = Some(token);
                }else if x == CustomTokenType::MacroEnd {
                    macro_end = Some(token);
                }
            }
        }

        //println!("[Debug] tokens: {:?}", &self.tokens);

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
        for call in macro_calls {
            self.insert_str(call.expand(&self.tabs).as_str());
            self.insert('\n');
            self.insert_str(self.tabs.clone().as_str());
        }

        let mut file = File::options()
            .read(false)
            .write(true)
            .truncate(true)
            .open(&self.path)
            .unwrap();

        let _ = file.write_all(self.string.as_bytes());

        Ok(())
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