use nalgebra::{matrix, ArrayStorage, Const, Matrix};

use super::Location;

type Button = char;

pub enum Space {
    Button(Button),
    Empty,
}

impl Space {
    pub fn button(&self) -> Option<&Button> {
        match self {
            Self::Button(button) => Some(button),
            Self::Empty => None,
        }
    }
}

type SquareMatrix<T, const S: usize> = Matrix<T, Const<S>, Const<S>, ArrayStorage<T, S, S>>;

pub struct Keypad<const S: usize>(SquareMatrix<Space, S>);

fn locations(height: usize, width: usize) -> impl Iterator<Item = Location> {
    (0..height).flat_map(move |row| (0..width).map(move |column| (row, column)))
}

impl<const S: usize> Keypad<S> {
    pub fn get(&self, location: Location) -> Option<&Button> {
        self.0.get(location).and_then(|space| space.button())
    }

    pub fn find_button(&self, target_button: Button) -> Option<Location> {
        let (height, width) = self.0.shape();
        locations(height, width).find(|&location| {
            self.get(location)
                .is_some_and(|&button| button == target_button)
        })
    }
}

pub const KEYPAD9: Keypad<3> = Keypad(matrix![
    Space::Button('1'), Space::Button('2'), Space::Button('3');
    Space::Button('4'), Space::Button('5'), Space::Button('6');
    Space::Button('7'), Space::Button('8'), Space::Button('9')
]);

pub const KEYPAD_WTF: Keypad<5> = Keypad(matrix![
    Space::Empty, Space::Empty, Space::Button('1'), Space::Empty, Space::Empty;
    Space::Empty, Space::Button('2'), Space::Button('3'), Space::Button('4'), Space::Empty;
    Space::Button('5'), Space::Button('6'), Space::Button('7'), Space::Button('8'), Space::Button('9');
    Space::Empty, Space::Button('A'), Space::Button('B'), Space::Button('C'), Space::Empty;
    Space::Empty, Space::Empty, Space::Button('D'), Space::Empty, Space::Empty;
]);
