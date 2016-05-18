pub struct Cell {
    unavailable_options: Vec<i32>
}

impl Cell {
    pub fn new() -> Cell {
        Cell { unavailable_options: vec!() }
    }

    pub fn options(&self) -> Vec<i32> {
        let mut options: Vec<i32> = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9);
        options.retain(|candidate: &i32|{ !self.unavailable_options.contains(candidate) });
        options
    }

    pub fn restrict(&mut self, option: i32) {
        self.unavailable_options.push(option);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_have_all_options_from_the_start() {
        let cell: Cell = Cell::new();

        assert_eq!(vec!(1,2,3,4,5,6,7,8,9), cell.options());
    }

    #[test]
    fn should_miss_options_when_restricted() {
        let mut cell: Cell = Cell::new();

        cell.restrict(9);

        assert_eq!(vec!(1,2,3,4,5,6,7,8), cell.options());
    }

    #[test]
    fn should_miss_multiple_options_when_restricted_multiple_times() {
        let mut cell: Cell = Cell::new();

        cell.restrict(9);
        cell.restrict(7);
        cell.restrict(6);

        assert_eq!(vec!(1,2,3,4,5,8), cell.options());
    }
}
