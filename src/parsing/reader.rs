use std::str::Chars;

pub struct Reader<'a> {
    pos: usize,
    col: usize,
    row: usize,
    input: Chars<'a>,
}

impl<'a> Iterator for Reader<'a> {
    type Item = char;
    fn next(&mut self) -> Option<char> {
        let next = self.input.next();
        self.pos += 1;

        if next == Some('\n') {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        next
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        Reader {
            pos: 0,
            col: 0,
            row: 0,
            input: input.chars(),
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }

    pub fn read_while(&mut self, f: impl Fn(char) -> bool) -> String {
        let mut s: String = String::new();

        while self.peek().map_or(false, &f) {
            s.push(self.next().unwrap());
        }

        s
    }
}
