pub struct Grid<'a> {
    src: &'a [u8],
    line_len: usize,
    lines: usize,
}

impl<'a> Grid<'a> {
    pub fn view(src: &'a str) -> Self {
        let src = src.as_bytes();
        let line_len = src
            .iter()
            .position(|&n| n == b'\n')
            .expect("no newline found in input string");
        let lines = src.len() / line_len;

        Self {
            src,
            line_len,
            lines,
        }
    }

    pub fn width(&self) -> usize {
        self.line_len
    }

    pub fn height(&self) -> usize {
        self.lines
    }

    pub fn at(&self, y: usize, x: usize) -> Option<u8> {
        self.src.get((y * self.line_len) + x + y).copied()
    }
}
