use std::fmt::Display;
use std::iter;
use std::ops::Index;

pub struct Grid<'a> {
    src: &'a str,
    line_len: usize,
    lines: usize,
}

fn line_len(src: &[u8]) -> usize {
    src.iter()
        .copied()
        .position(|n| n == b'\n')
        .expect("no newline found in input string")
}

impl<'a> Grid<'a> {
    pub fn view(src: &'a str) -> Self {
        let src = src.trim();
        let line_len = line_len(src.as_bytes());
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

    pub fn normalize_index(&self, y: usize, x: usize) -> usize {
        (y * self.line_len) + x + y
    }

    pub fn at(&self, y: usize, x: usize) -> Option<u8> {
        if x >= self.width() {
            None
        } else {
            self.src.as_bytes().get(self.normalize_index(y, x)).copied()
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        (0..self.height())
            .flat_map(move |y| (0..self.width()).map(move |x| (y, x, self.at(y, x).unwrap())))
    }

    pub fn direct_neighbors_opt(&self, y: usize, x: usize) -> [Option<(usize, usize, u8)>; 4] {
        direct_neighbor_indices(y, x).map(|(y, x, _)| {
            if let Ok(y) = usize::try_from(y)
                && let Ok(x) = usize::try_from(x)
            {
                self.at(y, x).map(|c| (y, x, c))
            } else {
                None
            }
        })
    }

    pub fn direct_neighbors_with_direction(
        &self,
        y: usize,
        x: usize,
    ) -> impl Iterator<Item = (usize, usize, u8, Direction)> + '_ {
        direct_neighbor_indices(y, x)
            .into_iter()
            .filter_map(|(y, x, dir)| {
                if let Ok(y) = usize::try_from(y)
                    && let Ok(x) = usize::try_from(x)
                {
                    self.at(y, x).map(|c| (y, x, c, dir))
                } else {
                    None
                }
            })
    }

    pub fn direct_neighbors(
        &self,
        y: usize,
        x: usize,
    ) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        self.direct_neighbors_opt(y, x).into_iter().flatten()
    }

    pub fn diag_iter(
        &self,
        y: usize,
        x: usize,
        diag: DiagDirection,
    ) -> impl Iterator<Item = (usize, usize, u8)> + '_ {
        let mut cursor = self.cursor(y, x);
        std::iter::from_fn(move || {
            let b = cursor.move_dir(diag)?;
            Some((cursor.y, cursor.x, b))
        })
    }

    pub fn cursor(&self, y: usize, x: usize) -> Cursor<'_, 'a> {
        Cursor { grid: self, x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Self::Down,
            Direction::Right => Self::Left,
            Direction::Down => Self::Up,
            Direction::Left => Self::Right,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum DiagDirection {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl From<Direction> for DiagDirection {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Self::Top,
            Direction::Right => Self::Right,
            Direction::Down => Self::Bottom,
            Direction::Left => Self::Left,
        }
    }
}

impl DiagDirection {
    pub const ALL: [Self; 8] = [
        Self::Top,
        Self::TopRight,
        Self::Right,
        Self::BottomRight,
        Self::Bottom,
        Self::BottomLeft,
        Self::Left,
        Self::TopLeft,
    ];

    pub fn opposite(self) -> Self {
        match self {
            DiagDirection::Top => DiagDirection::Bottom,
            DiagDirection::TopRight => DiagDirection::BottomLeft,
            DiagDirection::Right => DiagDirection::Left,
            DiagDirection::BottomRight => DiagDirection::TopLeft,
            DiagDirection::Bottom => DiagDirection::Top,
            DiagDirection::BottomLeft => DiagDirection::TopRight,
            DiagDirection::Left => DiagDirection::Right,
            DiagDirection::TopLeft => DiagDirection::BottomRight,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Index<Coord> for Grid<'_> {
    type Output = u8;
    fn index(&self, index: Coord) -> &Self::Output {
        &self.src.as_bytes()[self.normalize_index(index.y, index.x)]
    }
}

#[derive(Clone)]
pub struct Cursor<'a, 'b> {
    grid: &'a Grid<'b>,
    pub x: usize,
    pub y: usize,
}
impl<'a, 'b> Cursor<'a, 'b> {
    pub fn up(&mut self) -> Option<u8> {
        if let Some(y) = self.y.checked_sub(1) {
            self.grid.at(y, self.x).inspect(|_| self.y = y)
        } else {
            None
        }
    }

    pub fn right(&mut self) -> Option<u8> {
        self.grid.at(self.y, self.x + 1).inspect(|_| self.x += 1)
    }

    pub fn down(&mut self) -> Option<u8> {
        self.grid.at(self.y + 1, self.x).inspect(|_| self.y += 1)
    }

    pub fn left(&mut self) -> Option<u8> {
        if let Some(x) = self.x.checked_sub(1) {
            self.grid.at(self.y, x).inspect(|_| self.x = x)
        } else {
            None
        }
    }

    pub fn up_right(&mut self) -> Option<u8> {
        if let Some(y) = self.y.checked_sub(1) {
            self.grid.at(y, self.x + 1).inspect(|_| {
                self.y = y;
                self.x += 1;
            })
        } else {
            None
        }
    }

    pub fn up_left(&mut self) -> Option<u8> {
        if let (Some(y), Some(x)) = (self.y.checked_sub(1), self.x.checked_sub(1)) {
            self.grid.at(y, x).inspect(|_| {
                self.y = y;
                self.x = x;
            })
        } else {
            None
        }
    }

    pub fn down_right(&mut self) -> Option<u8> {
        self.grid.at(self.y + 1, self.x + 1).inspect(|_| {
            self.y += 1;
            self.x += 1;
        })
    }

    pub fn down_left(&mut self) -> Option<u8> {
        if let Some(x) = self.x.checked_sub(1) {
            self.grid.at(self.y + 1, x).inspect(|_| {
                self.y += 1;
                self.x = x;
            })
        } else {
            None
        }
    }

    pub fn move_dir(&mut self, dir: DiagDirection) -> Option<u8> {
        match dir {
            DiagDirection::Top => self.up(),
            DiagDirection::TopRight => self.up_right(),
            DiagDirection::Right => self.right(),
            DiagDirection::BottomRight => self.down_right(),
            DiagDirection::Bottom => self.down(),
            DiagDirection::BottomLeft => self.down_left(),
            DiagDirection::Left => self.left(),
            DiagDirection::TopLeft => self.up_left(),
        }
    }

    pub fn diag_iter(&mut self, dir: DiagDirection) -> impl Iterator<Item = u8> + use<'_, 'a, 'b> {
        iter::from_fn(move || self.move_dir(dir))
    }

    pub fn diag_pos_iter(
        &mut self,
        dir: DiagDirection,
    ) -> impl Iterator<Item = (usize, usize, u8)> + use<'_, 'a, 'b> {
        iter::from_fn(move || {
            let b = self.move_dir(dir)?;
            Some((self.y, self.x, b))
        })
    }
}

impl Display for Grid<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.src.fmt(f)
    }
}

pub fn direct_neighbor_indices(y: usize, x: usize) -> [(isize, isize, Direction); 4] {
    let y = y as isize;
    let x = x as isize;
    [
        (y - 1, x, Direction::Up),
        (y, x + 1, Direction::Right),
        (y + 1, x, Direction::Down),
        (y, x - 1, Direction::Left),
    ]
}

pub struct MutableGrid<'a> {
    // invariant: always valid utf-8
    src: &'a mut [u8],
    line_len: usize,
    lines: usize,
}

impl<'a> MutableGrid<'a> {
    pub fn new(src: &'a mut [u8]) -> Self {
        assert!(std::str::from_utf8(src).is_ok());
        let line_len = line_len(&*src);
        let lines = src.len() / line_len;
        Self {
            src,
            line_len,
            lines,
        }
    }

    pub fn imm(&self) -> Grid<'_> {
        Grid {
            line_len: self.line_len,
            lines: self.lines,
            src: unsafe { std::str::from_utf8_unchecked(self.src) },
        }
    }

    pub fn set(&mut self, y: usize, x: usize, b: u8) {
        assert!(b <= 127);
        let i = self.imm().normalize_index(y, x);
        self.src[i] = b;
    }

    pub fn at(&self, y: usize, x: usize) -> Option<u8> {
        self.imm().at(y, x)
    }

    pub fn swap(&mut self, ny: usize, nx: usize, y: usize, x: usize) {
        let i1 = self.imm().normalize_index(ny, nx);
        let i2 = self.imm().normalize_index(y, x);
        assert!(self.src[i1] <= 127);
        assert!(self.src[i2] <= 127);
        self.src.swap(i1, i2);
    }
}
