use std::ops::Index;
use super::cell::Cell;

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new() -> Grid {
        let mut cells = vec!();
        for _ in 0..81 {
            cells.push(Cell::new());
        }
        Grid {
            cells: cells,
        }
    }
}

impl Index<(i32,i32)> for Grid {
    type Output = Cell;

    fn index<'a>(&'a self, tuple: (i32, i32)) -> &'a Cell {
        &self.cells[(9 * tuple.0 + tuple.1) as usize]
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_index_cells() {
        let grid: Grid = Grid::new();

        let ref cell = grid[(8, 8)];

        assert_eq!(vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), cell.options());
    }

    #[test]
    fn should_iterate_over_cells() {
        let grid: Grid = Grid::new();
        let mut cell_count = 0;

        for _ in grid.cells {
           cell_count += 1;
        }

        assert_eq!(81, cell_count);
    }
}
