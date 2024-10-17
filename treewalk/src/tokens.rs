struct Token<'a> {
    typ: TokenType,
    lexeme: &'a str,
    literal: Object,
    line: i32,
}

impl Token<'_> {
    fn new(t: TokenType, lex: &str, lit: Object, lin: i32) -> Self {
        Self {
            typ: t,
            lexeme: lex,
            literal: lit,
            line: lin,
        }
    }

    fn to_string<'a>(self) -> &'a str {
        let mut string = String::from(self.typ.to_string());
        string.push_str(" ");
        string.push_str(&self.lexeme);
        string.push_str(" ");
        string.push_str(self.literal.to_string());

        return string.as_str();
    }
}
