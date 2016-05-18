enum Binding {
    Unbound,
    Bound(i32),
}

pub struct Cell {
    unavailable_options: Vec<i32>,
    binding: Binding,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            unavailable_options: vec!(),
            binding: Binding::Unbound,
        }
    }

    pub fn options(&self) -> Vec<i32> {
        match self.binding {
            Binding::Unbound  => {
                let mut options: Vec<i32> = vec!(1, 2, 3, 4, 5, 6, 7, 8, 9);
                options.retain(|candidate: &i32|{ !self.unavailable_options.contains(candidate) });
                options
            }
            Binding::Bound(_) => {
                vec!()
            }
        }
    }

    pub fn restrict(&mut self, option: i32) {
        self.unavailable_options.push(option);
    }

    pub fn assign(&mut self, choice: i32) {
        self.binding = Binding::Bound(choice);
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

    #[test]
    fn should_have_no_options_when_assigned() {
        let mut cell: Cell = Cell::new();
        let no_options: Vec<i32> = vec!();

        cell.assign(1);

        assert_eq!(no_options, cell.options());
    }
}
