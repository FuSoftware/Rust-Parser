pub struct Reader {
    pos: usize,
    col: usize,
    row: usize,
    input: String
}

impl Reader {
    pub fn new(input: String) -> Reader {
        Reader {
            pos: 0,
            col: 0,
            row: 0,
            input,
        }
    }

    pub fn next(&mut self) -> char {
        let c: char = self.peek();
        self.pos += 1;

        if c == '\n' {
            self.row += 1;
            self.col = 0;
        }else {
            self.col += 1;
        }
        
        c
    }

    pub fn peek(&self) -> char {
        self.peek_offset(0)
    }

    pub fn peek_offset(&self, offset: usize) -> char {
        self.input.chars().nth(self.pos + offset).unwrap()
    }

    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    pub fn remaining(&self) -> usize {
        self.input.len() - self.pos
    }

    pub fn clear(&mut self) {
        self.pos = 0;
        self.row = 0;
        self.col = 0;
        self.input = String::from("");
    }
}
