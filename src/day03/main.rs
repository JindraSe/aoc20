use std::{ops::{Index, Add}, fs::read_to_string, path::Path, env::args};

struct Position {
    x: usize,
    y: usize,
}

struct Slope {
    x: usize,
    y: usize,
}

impl Add<&Slope> for Position {
    type Output = Self;

    fn add(self, other: &Slope) -> Position {
        return Position { x: self.x + other.x, y: self.y + other.y }
    }
}

struct TreeLine {
    elems: Vec<bool>,
}

impl TreeLine {
    fn width(&self) -> usize {
        return self.elems.len();
    }

    fn from_line(line: &str) -> TreeLine {
        return TreeLine {
            elems: line.chars().map(|ch| ch == '#').collect(),
        }
    }
}

struct TreeGrid {
    height: usize,
    base_width: usize,
    elems: Vec<bool>,
}

impl TreeGrid {
    fn append(&mut self, next: &TreeLine) {
        if self.height == 0 {
            self.base_width = next.width();
        }

        if next.width() != self.base_width {
            panic!("Tree line width does not match tree grid width");
        }

        self.elems.extend(&next.elems);
        self.height += 1;
    }

    fn new() -> TreeGrid {
        return TreeGrid {
            height: 0,
            base_width: 0,
            elems: Vec::new(),
        }
    }

    fn from_lines<I>(lines: I) -> TreeGrid 
    where I: Iterator<Item = TreeLine> {
        let mut grid = TreeGrid::new();

        for line in lines {
            grid.append(&line);
        }

        return grid;
    }
}

impl Index<&Position> for TreeGrid {
    type Output = bool;

    fn index(&self, p: &Position) -> &bool {
        let x_reduced = p.x % self.base_width;

        if p.y > self.height {
            panic!("Index out of bounds");
        }

        return &self.elems[p.y*self.base_width + x_reduced];
    }
}

fn read_grid(path: &Path) -> TreeGrid {
    return TreeGrid::from_lines(read_to_string(path)
        .expect("Input file not found")
        .lines()
        .filter(|l| !l.is_empty())
        .map(TreeLine::from_line));
}

fn count_trees(grid: &TreeGrid, slope: &Slope) -> u32 {
    let mut current_position: Position = Position { x: 0, y: 0 };
    let mut seen_trees: u32 = 0;

    while current_position.y < grid.height {
        seen_trees += if grid[&current_position] { 1 } else { 0 };
        current_position = current_position + &slope;
    }

    return seen_trees;
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        panic!("Expected an argument");
    }

    let path_to_input = Path::new(&args[1]);
    let grid = read_grid(path_to_input);

    let tree_count = count_trees(&grid, &Slope { x: 3, y: 1 });

    let interesting_slopes = vec![
        Slope { x: 1, y: 1 },
        Slope { x: 3, y: 1 },
        Slope { x: 5, y: 1 },
        Slope { x: 7, y: 1 },
        Slope { x: 1, y: 2 }
    ];

    let interesting_slopes_product: u32 = interesting_slopes
        .iter()
        .map(|is| count_trees(&grid, is))
        .product();

    println!("Counted {} trees!", tree_count);
    println!("Product of tree counts for interesting slopes: {}!", interesting_slopes_product);
}

