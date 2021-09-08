pub struct Lexer<'a> {
    src: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Lexer {
        Lexer {
            src,
            index: 0
        }
    }
}

impl Lexer<'_> {
    pub fn read_char(&mut self) -> Option<char> {
        self.src[self.index..].chars().next()
    }

    pub fn next_char(&mut self) {
        self.index += 1;

        while !self.src.is_char_boundary(self.index) {
            self.index += 1;
        }
    }

    pub fn next_char_while(&mut self, func: impl Fn(char) -> bool) {
        while self.read_char().map(&func) == Some(true) {
            self.next_char();
        }
    }
}

type Token<'a> = &'a str;

impl<'a> Lexer<'a> {
    pub fn next_tokn(&mut self) -> Option<Token<'a>> {
        self.next_char_while(|c| c.is_whitespace());

        let start = self.index;

        self.next_char_while(|c| !c.is_whitespace());

        let end = self.index;

        if start == end {
            return None;
        }

        return Some(&self.src[start..end]);
    }

    pub fn next_tokn_if(&mut self, func: impl FnOnce(Token) -> bool) -> Option<Token<'a>> {
        let save = self.index;

        if let Some(tokn) = self.next_tokn() {
            if func(tokn) {
                return Some(tokn);
            }
        }

        self.index = save;

        return None;
    }

    pub fn next_tokn_map_if<T>(&mut self, func: impl FnOnce(Token) -> Option<T>) -> Option<T> {
        let save = self.index;

        if let Some(tokn) = self.next_tokn() {
            if let Some(value) = func(tokn) {
                return Some(value);
            }
        }

        self.index = save;

        return None;
    }
}