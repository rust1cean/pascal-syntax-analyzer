pub mod lexer {
    use anyhow::Result;

    use crate::token::{Position, Token};

    pub fn read_as_tokens(input: &str) -> anyhow::Result<Box<[Token]>> {
        let mut tokens = Vec::<Token>::new();

        let mut mode = Mode::Normal;
        let mut from = 0;
        let mut to = 0;

        let slice = || &input[from..=to];

        for c in input.chars() {
            match mode {
                Mode::Normal => match c {
                    c if c.is_whitespace() && from == to => {
                        to += 1;
                    }
                    c if c.is_whitespace() => {
                        // Validate token
                        from = to;
                    }
                    c if c.is_alphabetic() || matches!(c, ':' | '<' | '>') => to += 1,
                    '\'' => mode = Mode::String,
                    _ => todo!(),
                },
                Mode::String => match c {
                    '\'' => mode = Mode::Normal,
                    _ => to += 1,
                },
                _ => (),
            }
        }

        Ok(tokens.into_boxed_slice())
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
}

pub mod token {
    use phf::phf_map;

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
            // Keyword::try_from(value)
            //     .map(Self::Keyword)
            //     .or_else(|_| Identifier::try_from(value).map(Self::Identifier))
            //     .or_else(|_| Literal::try_from(value).map(Self::Literal))
            //     .or_else(|_| Operator::try_from(value).map(Self::Operator))
            //     .or_else(|_| SpecialSymbol::try_from(value).map(Self::SpecialSymbol))
            todo!()
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

    pub fn parse_special_symbol(input: &str) -> Option<SpecialSymbol> {
        SPECIAL_SYMBOLS.get(input).cloned()
    }

    static SPECIAL_SYMBOLS: phf::Map<&'static str, SpecialSymbol> = phf_map! {
        ":=" => SpecialSymbol::Assign,
        ".." => SpecialSymbol::Range,
        "." => SpecialSymbol::Dot,
        ":" => SpecialSymbol::Colon,
        ";" => SpecialSymbol::Semicolon,
        "," => SpecialSymbol::Comma,
        "(" => SpecialSymbol::LeftParenthesis,
        ")" => SpecialSymbol::RightParenthesis,
        "[" => SpecialSymbol::LeftBracket,
        "]" => SpecialSymbol::RightBracket,

    };

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
