pub mod lexer {
    use std::str::Chars;

    use anyhow::Result;

    use crate::symbols::*;
    use crate::token::{Position, Token};

    #[derive(Debug, Default, PartialEq, Eq, Clone)]
    pub struct Lexer {
        mode: Mode,
    }

    impl Lexer {
        pub fn read_as_tokens(&mut self, input: &str) -> anyhow::Result<Box<[Token]>> {
            let mut tokens = Vec::<Token>::new();
            let mut span = Span::new(input);

            // On each iteration:
            // process char
            // error -> can't process char
            //
            // match token
            // ready -> add token
            // pending -> add char
            // invalid -> error

            for token in span {
                // println!(
                //     "({:>2}:{:<3}) {c:?}",
                //     self.position.line, self.position.column
                // );

                println!("{token:?}");
            }

            Ok(tokens.into_boxed_slice())
        }
    }

    #[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
    enum Mode {
        #[default]
        Normal,
        String,
        Number,
        Comment,
        Operator,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Span<'text> {
        input: &'text str,
        shift: usize,
        len: usize,
    }

    impl Iterator for Span<'_> {
        type Item = ();

        fn next(&mut self) -> Option<Self::Item> {
            self.shift = self.end() + 1;
            self.len = 1;

            (self.shift < self.len).then_some(())
        }
    }

    impl<'text> Span<'text> {
        pub fn new(input: &'text str) -> Self {
            let shift = 0;
            let len = 1;

            Self { input, shift, len }
        }

        pub fn is_something_on_the_left(&self) -> Option<char> {
            self.input[self.shift..].chars().next()
        }

        pub fn is_something_on_the_right(&self) -> Option<char> {
            self.input[self.end()..].chars().next()
        }

        pub fn shift_while(&mut self, cb: fn(c: char) -> bool) {
            while let Some(c) = self.is_something_on_the_left() {
                match cb(c) {
                    true => self.shift_by_one(),
                    false => break,
                }
            }
        }

        pub fn extend_while(&mut self, cb: fn(c: char) -> bool) {
            while let Some(c) = self.is_something_on_the_right() {
                match cb(c) {
                    true => self.extend_by_one(),
                    false => break,
                }
            }
        }

        pub fn get(&self) -> &str {
            &self.input[self.shift..self.end()]
        }

        fn shift_by_one(&mut self) {
            self.shift += 1;
        }

        fn extend_by_one(&mut self) {
            self.len += 1;
        }

        fn end(&self) -> usize {
            let Self { input, shift, len } = *self;
            let end = shift + len;

            end.min(input.len())
        }
    }
}

pub mod token {
    use crate::symbols::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Token {
        value: &'static str,
        kind: TokenKind,
        position: Position,
    }

    impl TryFrom<&'static str> for Token {
        type Error = &'static str;

        fn try_from(value: &'static str) -> Result<Self, Self::Error> {
            let kind = TokenKind::try_from(value)?;
            let position = Position::default();

            Ok(Self {
                value,
                kind,
                position,
            })
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum TokenKind {
        Identifier(Identifier),
        Literal(Literal),
        Keyword(Keyword),
        Operator(Operator),
        SpecialSymbol(SpecialSymbol),
    }

    impl TryFrom<&str> for TokenKind {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Keyword::try_from(value)
                .map(Self::Keyword)
                .or_else(|_| Identifier::try_from(value).map(Self::Identifier))
                .or_else(|_| Literal::try_from(value).map(Self::Literal))
                .or_else(|_| Operator::try_from(value).map(Self::Operator))
                .or_else(|_| SpecialSymbol::try_from(value).map(Self::SpecialSymbol))
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Identifier;

    impl TryFrom<&str> for Identifier {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let first_is_alphabetic = value
                .chars()
                .nth(0)
                .ok_or("Expected identifier, got empty string.")?
                .is_ascii_alphabetic();

            let other_chars_are_alphanumeric = value[1..].chars().all(|c| c.is_alphanumeric());

            match first_is_alphabetic && other_chars_are_alphanumeric {
                true => Ok(Self),
                false => Err("Can't parse identifier"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum Literal {
        String,
        Number,
        Boolean,
    }

    impl TryFrom<&str> for Literal {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                x if Self::is_string(x) => Ok(Self::String),
                x if Self::is_number(x) => Ok(Self::Number),
                x if Self::is_boolean(x) => Ok(Self::Boolean),
                _ => Err("Can't recognize literal type"),
            }
        }
    }

    impl Literal {
        fn is_string(value: &str) -> bool {
            value
                .chars()
                .all(|c| c.is_ascii_alphabetic() || c == APOSTROPHE)
        }

        fn is_number(value: &str) -> bool {
            value
                .chars()
                .all(|c| c.is_ascii_digit() || c.is_ascii_hexdigit() || c == DOT)
        }

        fn is_boolean(value: &str) -> bool {
            value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("false")
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum SpecialSymbol {
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

    impl TryFrom<&str> for SpecialSymbol {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                ":=" => Ok(Self::Assign),
                ".." => Ok(Self::Range),
                "." => Ok(Self::Dot),
                ":" => Ok(Self::Colon),
                ";" => Ok(Self::Semicolon),
                "," => Ok(Self::Comma),
                "(" => Ok(Self::LeftParenthesis),
                ")" => Ok(Self::RightParenthesis),
                "[" => Ok(Self::LeftBracket),
                "]" => Ok(Self::RightBracket),
                _ => Err("Can't recognize special symbol."),
            }
        }
    }

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

    impl TryFrom<&str> for Keyword {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                x if x.eq_ignore_ascii_case("begin") => Ok(Self::Begin),
                x if x.eq_ignore_ascii_case("end") => Ok(Self::End),
                x if x.eq_ignore_ascii_case("if") => Ok(Self::If),
                x if x.eq_ignore_ascii_case("for") => Ok(Self::For),
                x if x.eq_ignore_ascii_case("then") => Ok(Self::Then),
                x if x.eq_ignore_ascii_case("else") => Ok(Self::Else),
                x if x.eq_ignore_ascii_case("break") => Ok(Self::Break),
                x if x.eq_ignore_ascii_case("continue") => Ok(Self::Continue),
                x if x.eq_ignore_ascii_case("do") => Ok(Self::Do),
                x if x.eq_ignore_ascii_case("repeat") => Ok(Self::Repeat),
                x if x.eq_ignore_ascii_case("of") => Ok(Self::Of),
                x if x.eq_ignore_ascii_case("var") => Ok(Self::Var),
                x if x.eq_ignore_ascii_case("until") => Ok(Self::Until),
                x if x.eq_ignore_ascii_case("type") => Ok(Self::Type),
                x if x.eq_ignore_ascii_case("const") => Ok(Self::Const),
                x if x.eq_ignore_ascii_case("function") => Ok(Self::Function),
                x if x.eq_ignore_ascii_case("procedure") => Ok(Self::Procedure),
                x if x.eq_ignore_ascii_case("program") => Ok(Self::Program),
                x if x.eq_ignore_ascii_case("true") => Ok(Self::True),
                x if x.eq_ignore_ascii_case("false") => Ok(Self::False),
                _ => Err("Can't recognize keyword."),
            }
        }
    }

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
        Equal,
        NotEqual,
        Less,
        LessOrEqual,
        Greater,
        GreaterOrEqual,

        // arithmetic
        Add,
        Sub,
        Mul,
        Div,
        Mod,
    }

    impl TryFrom<&str> for Operator {
        type Error = &'static str;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "<<" => Ok(Self::ShiftLeft),
                ">>" => Ok(Self::ShiftRight),
                "==" => Ok(Self::Equal),
                "!=" => Ok(Self::NotEqual),
                "<=" => Ok(Self::LessOrEqual),
                ">=" => Ok(Self::GreaterOrEqual),
                "<" => Ok(Self::Less),
                ">" => Ok(Self::Greater),
                "+" => Ok(Self::Add),
                "-" => Ok(Self::Sub),
                "*" => Ok(Self::Mul),
                "/" => Ok(Self::Div),
                "%" => Ok(Self::Mod),
                x if x.eq_ignore_ascii_case("and") => Ok(Self::And),
                x if x.eq_ignore_ascii_case("or") => Ok(Self::Or),
                x if x.eq_ignore_ascii_case("not") => Ok(Self::Not),
                x if x.eq_ignore_ascii_case("xor") => Ok(Self::Xor),
                _ => Err("Can't recognize operator."),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct Position {
        pub line: usize,
        pub column: usize,
    }

    impl Default for Position {
        fn default() -> Self {
            Self {
                line: Self::DEFAULT_LINE,
                column: Self::DEFAULT_COLUMN,
            }
        }
    }

    impl Position {
        const DEFAULT_COLUMN: usize = 1;
        const DEFAULT_LINE: usize = 1;

        pub fn update(&mut self, c: char) {
            match c {
                '\n' => {
                    self.column = Self::DEFAULT_COLUMN;
                    self.line += 1;
                }
                _ => self.column += 1,
            }
        }
    }
}

pub mod symbols {
    pub const DOT: char = '.';
    pub const COLON: char = ':';
    pub const SEMICOLON: char = ';';
    pub const PLUS: char = '+';
    pub const MINUS: char = '-';
    pub const ASTERISK: char = '*';
    pub const SOLIDUS: char = '/';
    pub const REVERSE_SOLIDUS: char = '\\';
    pub const PERCENT: char = '%';
    pub const LESS: char = '<';
    pub const GREATER: char = '>';
    pub const COMMA: char = ',';
    pub const EQUALS: char = '=';
    pub const APOSTROPHE: char = '\'';
    pub const EXCLAMATION: char = '!';
    pub const LEFT_PARENT: char = '(';
    pub const RIGHT_PARENT: char = ')';
    pub const LEFT_BRACKET: char = '[';
    pub const RIGHT_BRACKET: char = ']';
    pub const LEFT_CURLY_BRACKET: char = '{';
    pub const RIGHT_CURLY_BRACKET: char = '}';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let input = r##"
{ It's a comment "_" }
program Simple;
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

        let mut lexer = lexer::Lexer::default();

        let _ = lexer.read_as_tokens(input);
    }
}
