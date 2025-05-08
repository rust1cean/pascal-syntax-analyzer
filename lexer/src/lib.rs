
    use crate::utils::Position;

    pub struct Token {
        kind: TokenKind,
        value: &'static str,
        position: Position,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TokenKind {
        Identifier,
        Commentary(Commentary),
        Literal(LiteralType),
        Keyword(Keyword),
        Operator(Operator),
        SpecialSymbol(SpecialSymbol),
    }

    impl TokenKind {
        pub fn try_identify(input: &'static str) -> Option<Self> {
            None
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Commentary {
        Commentary,
    }

    impl Commentary {
        pub fn is_commentary(input: &'static str) -> Option<Self> {
            match input {
                "//" | "{" | "}" | "(*" | "*)" => Some(Self::Commentary),
                _ => None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum LiteralType {
        String,
        Integer,
        Real,
        Char,
        Boolean,
    }

    impl LiteralType {
        pub fn is_literal(input: &'static str) -> Option<Self> {
            match input {
                x if x.eq_ignore_ascii_case("string") => Some(Self::String),
                x if x.eq_ignore_ascii_case("integer") => Some(Self::Integer),
                x if x.eq_ignore_ascii_case("real") => Some(Self::Real),
                x if x.eq_ignore_ascii_case("char") => Some(Self::Char),
                x if x.eq_ignore_ascii_case("boolean") => Some(Self::Boolean),
                _ => None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
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

    impl SpecialSymbol {
        pub fn is_special_Symbol(input: &'static str) -> Option<Self> {
            match input {
                ":=" => Some(Self::Assign),
                ".." => Some(Self::Range),
                "." => Some(Self::Dot),
                ":" => Some(Self::Colon),
                ";" => Some(Self::Semicolon),
                "," => Some(Self::Comma),
                "(" => Some(Self::LeftParenthesis),
                ")" => Some(Self::RightParenthesis),
                "[" => Some(Self::LeftBracket),
                "]" => Some(Self::RightBracket),
                _ => None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
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

    impl Keyword {
        pub fn is_keyword(input: &'static str) -> Option<Self> {
            match input {
                x if x.eq_ignore_ascii_case("begin") => Some(Self::Begin),
                x if x.eq_ignore_ascii_case("end") => Some(Self::End),

                x if x.eq_ignore_ascii_case("if") => Some(Self::If),
                x if x.eq_ignore_ascii_case("for") => Some(Self::For),
                x if x.eq_ignore_ascii_case("then") => Some(Self::Then),
                x if x.eq_ignore_ascii_case("else") => Some(Self::Else),
                x if x.eq_ignore_ascii_case("break") => Some(Self::Break),
                x if x.eq_ignore_ascii_case("continue") => Some(Self::Continue),

                x if x.eq_ignore_ascii_case("var") => Some(Self::Var),
                x if x.eq_ignore_ascii_case("const") => Some(Self::Const),
                x if x.eq_ignore_ascii_case("function") => Some(Self::Function),
                x if x.eq_ignore_ascii_case("procedure") => Some(Self::Procedure),
                x if x.eq_ignore_ascii_case("program") => Some(Self::Program),

                x if x.eq_ignore_ascii_case("true") => Some(Self::True),
                x if x.eq_ignore_ascii_case("false") => Some(Self::False),

                _ => None,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
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

    impl Operator {
        pub fn is_operator(input: &'static str) -> Option<Self> {
            match input {
                "<<" => Some(Self::ShiftLeft),
                ">>" => Some(Self::ShiftRight),

                "==" => Some(Self::Equal),
                "!=" => Some(Self::NotEqual),
                "<=" => Some(Self::LessOrEqual),
                ">=" => Some(Self::GreaterOrEqual),
                "<" => Some(Self::Less),
                ">" => Some(Self::Greater),

                "+" => Some(Self::Add),
                "-" => Some(Self::Sub),
                "*" => Some(Self::Mul),
                "/" => Some(Self::Div),
                "%" => Some(Self::Mod),

                x if x.eq_ignore_ascii_case("and") => Some(Self::And),
                x if x.eq_ignore_ascii_case("or") => Some(Self::Or),
                x if x.eq_ignore_ascii_case("not") => Some(Self::Not),
                x if x.eq_ignore_ascii_case("xor") => Some(Self::Xor),

                _ => None,
            }
        }
    }

pub struct Position {
        line: u32,
        column: u32,
    }
