const TEST: &'static str = r#"
   program Hello;
   begin
     writeln('Hello, world!');
   end.
"#;

fn main() {
    assert!(is_valid_code(TEST));
}

fn is_valid_code(input: &'static str) -> bool {}

mod parser {
    pub struct Parser {
        input: &'static str,
    }

    impl Parser {
        pub fn parse(input: &'static str) {
            let tokens = input.split(" ");
        }
    }
}

mod utils {
    
}
