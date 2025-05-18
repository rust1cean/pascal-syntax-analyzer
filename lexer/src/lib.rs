pub mod lexer {
    use std::ops::Range;

    use anyhow::Result;

    use crate::token::Identifier;
    use crate::token::Keyword;
    use crate::token::Literal;
    use crate::token::Position;
    use crate::token::Symbol;
    use crate::token::Token;
    use crate::token::TokenKind;

    pub fn read_as_tokens(input: &str) -> anyhow::Result<Box<[Token]>> {
        let mut tokens = Vec::<Token>::new();

        let mut mode = Mode::Normal;
        let mut from = 0;
        let mut to = 0;

        let slice = || &input[from..=to];

        // A..z
        // 0..9
        // "'+-*/%<>=.;:()[]

        for c in input.chars() {}

        Ok(tokens.into_boxed_slice())
    }

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    pub struct Lexer {
        mode: Mode,
        range: Range<usize>,
        position: Position,
    }

    impl<'token> Lexer {
        pub fn parse_as_tokens(
            &mut self,
            input: &'token str,
        ) -> anyhow::Result<Box<[Token<'token>]>> {
            let mut tokens = Vec::<Token<'token>>::new();

            for c in input.chars() {
                let token = match self.mode {
                    Mode::Normal => self.process_as_normal(c, input)?.into(),
                    Mode::String => self.process_as_string(c, input),
                    Mode::Number => self.process_as_number(c, input),
                    Mode::Comment => {
                        self.process_as_comment(c, input);
                        None
                    }
                };

                if let Some(t) = token {
                    tokens.push(t);
                }
            }

            Ok(tokens.into_boxed_slice())
        }

        fn process_as_normal(
            &mut self,
            c: char,
            input: &'token str,
        ) -> anyhow::Result<Token<'token>> {
            match c {
                '\'' => self.mode.set(Mode::String),
                '{' => self.mode.set(Mode::Comment),
            }

            todo!()
        }

        fn process_as_string(&mut self, c: char, input: &'token str) -> Option<Token<'token>> {
            match c {
                '\'' => {
                    self.mode.set(Mode::Normal);

                    let value = self.get_slice(input);
                    let kind = TokenKind::Literal(Literal::String);
                    let position = self.position;

                    Some(Token::new(value, kind, position))
                }
                _ => {
                    self.range.end += 1;
                    None
                }
            }
        }

        fn process_as_number(&mut self, c: char, input: &'token str) -> Option<Token<'token>> {
            None
        }

        fn process_as_comment(&self, c: char, input: &'token str) {
            if c == '}' {
                self.mode.set(Mode::Normal);
            }
        }

        fn get_slice(&self, input: &'token str) -> &'token str {
            &input[self.range.clone()]
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    enum Mode {
        #[default]
        Normal,
        Number,
        String,
        Comment,
    }

    impl Mode {
        pub fn toggle(&mut self, mode: Self) {
            match *self == mode {
                true => self.set(Self::Normal),
                false => self.set(mode),
            }
        }

        pub fn set(&mut self, mode: Self) {
            *self = mode;
        }
    }
}

pub mod token {
    use phf::phf_map;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Token<'token> {
        value: &'token str,
        kind: TokenKind,
        position: Position,
    }

    impl<'token> TryFrom<&'static str> for Token<'token> {
        type Error = &'static str;

        fn try_from(value: &'token str) -> Result<Self, Self::Error> {
            let kind = TokenKind::try_from(value)?;
            let position = Position::default();

            Ok(Self {
                value,
                kind,
                position,
            })
        }
    }

    impl<'token> Token<'token> {
        pub fn new(value: &'token str, kind: TokenKind, position: Position) -> Self {
            Self {
                value,
                kind,
                position,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum TokenKind {
        Identifier(Identifier),
        Literal(Literal),
        Keyword(Keyword),
        Operator(Operator),
        Symbol(Symbol),
    }

    impl TryFrom<&str> for TokenKind {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            if let Some(literal) = Literal::parse(value) {
                return Ok(Self::Literal(literal));
            }

            if let Some(keyword) = parse_keyword(value) {
                return Ok(Self::Keyword(keyword));
            }

            if let Some(symbol) = parse_symbol(value) {
                return Ok(Self::Symbol(symbol));
            }

            if let Some(operator) = parse_operator(value) {
                return Ok(Self::Operator(operator));
            }

            Identifier::try_from(value).map(Self::Identifier)
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Identifier;

    impl TryFrom<&str> for Identifier {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let mut chars = value.chars();

            let first_is_alphabetic = chars.next().is_some_and(|c| c.is_ascii_alphabetic());

            match first_is_alphabetic && chars.all(|c| c.is_ascii_alphabetic()) {
                true => Ok(Self),
                false => Err("Invalid name of identfier."),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum Literal {
        String,
        Int,
        Float,
        Char,
        Bool,
    }

    impl Literal {
        pub fn parse(value: &str) -> Option<Self> {
            match value {
                x if Self::is_char(x) => Some(Self::Char),
                x if Self::is_bool(x) => Some(Self::Bool),
                x if Self::is_int(x) => Some(Self::Int),
                x if Self::is_float(x) => Some(Self::Float),
                x if Self::is_string(x) => Some(Self::String),
                _ => None,
            }
        }

        fn is_char(value: &str) -> bool {
            value
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphanumeric())
        }

        fn is_string(value: &str) -> bool {
            value.chars().all(|c| c.is_ascii_alphabetic())
        }

        fn is_int(value: &str) -> bool {
            value
                .chars()
                .all(|c| c.is_ascii_digit() || c.is_ascii_hexdigit())
        }

        fn is_float(value: &str) -> bool {
            value
                .chars()
                .all(|c| c.is_ascii_digit() || c.is_ascii_hexdigit() || c == '.')
        }

        fn is_bool(value: &str) -> bool {
            value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false")
        }
    }

    pub fn parse_symbol(input: &str) -> Option<Symbol> {
        SYMBOLS.get(input).cloned()
    }

    static SYMBOLS: phf::Map<&'static str, Symbol> = phf_map! {
        ":=" => Symbol::Assign,
        ".." => Symbol::Range,
        "." => Symbol::Dot,
        ":" => Symbol::Colon,
        ";" => Symbol::Semicolon,
        "," => Symbol::Comma,
        "(" => Symbol::LeftParenthesis,
        ")" => Symbol::RightParenthesis,
        "[" => Symbol::LeftBracket,
        "]" => Symbol::RightBracket,
    };

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum Symbol {
        Dot,
        Colon,
        Semicolon,
        Assign,
        Range,
        Comma,
        LeftParenthesis,
        RightParenthesis,
        LeftBracket,
        RightBracket,
    }

    pub fn parse_keyword(input: &str) -> Option<Keyword> {
        KEYWORDS.get(input).cloned()
    }

    static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
        "begin" => Keyword::Begin,
        "end" => Keyword::End,
        "if" => Keyword::If,
        "for" => Keyword::For,
        "then" => Keyword::Then,
        "else" => Keyword::Else,
        "break" => Keyword::Break,
        "continue" => Keyword::Continue,
        "do" => Keyword::Do,
        "repeat" => Keyword::Repeat,
        "of" => Keyword::Of,
        "var" => Keyword::Var,
        "until" => Keyword::Until,
        "type" => Keyword::Type,
        "const" => Keyword::Const,
        "function" => Keyword::Function,
        "procedure" => Keyword::Procedure,
        "program" => Keyword::Program,
        "true" => Keyword::True,
        "false" => Keyword::False,
    };

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum Keyword {
        // scope
        Begin,
        End,

        // control flow
        If,
        For,
        Then,
        Else,
        Break,
        Continue,
        Do,
        Of,
        Repeat,
        Until,

        // declare
        Type,
        Var,
        Const,
        Function,
        Procedure,
        Program,

        // boolean
        True,
        False,
    }

    pub fn parse_operator(input: &str) -> Option<Operator> {
        OPERATORS.get(input).cloned()
    }

    static OPERATORS: phf::Map<&'static str, Operator> = phf_map! {
        "shl" => Operator::ShiftLeft,
        "shr" => Operator::ShiftRight,
        "==" => Operator::Equals,
        "<>" => Operator::NotEquals,
        "<=" => Operator::LessOrEquals,
        ">=" => Operator::GreaterOrEquals,
        "<" => Operator::Less,
        ">" => Operator::Greater,
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "%" => Operator::Mod,
        "and" => Operator::And,
        "or" => Operator::Or,
        "not" => Operator::Not,
        "xor" => Operator::Xor,
    };

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum Operator {
        // bitwise
        ShiftLeft,
        ShiftRight,

        // logical
        And,
        Or,
        Xor,
        Not,

        // relational
        Equals,
        NotEquals,
        Less,
        LessOrEquals,
        Greater,
        GreaterOrEquals,

        // arithmetic
        Add,
        Sub,
        Mul,
        Div,
        Mod,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Position {
        pub line: usize,
        pub column: usize,
    }

    impl Default for Position {
        fn default() -> Self {
            Self { line: 1, column: 1 }
        }
    }

    impl Position {
        pub fn add_column(&mut self) {
            self.column += 1;
        }

        pub fn add_line(&mut self) {
            self.line += 1;
            self.column = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let input = r##"
{ It's a comment "_" }
program MyProgram;
Var
  A, B: Integer;
  C: Integer;
begin
  A := 1;
  B := 2;
  C := A + B;
  
  writeln(A, ' + ', B, ' = ', C);
end."##;

        dbg!(input);

        let _ = lexer::read_as_tokens(input);
    }
}
