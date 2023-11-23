use nalgebra::{matrix, Matrix3, Matrix5};

pub const KEYPAD9: Matrix3<char> = matrix![
    '1', '2', '3';
    '4', '5', '6';
    '7', '8', '9'
];

pub const KEYPAD_WTF: Matrix5<Option<char>> = matrix![
    None, None, Some('1'), None, None;
    None, Some('2'), Some('3'), Some('4'), None;
    Some('5'), Some('6'), Some('7'), Some('8'), Some('9');
    None, Some('A'), Some('B'), Some('C'), None;
    None, None, Some('D'), None, None;
];
