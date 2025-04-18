/// Utility functions for converting chess notation to indices.
pub fn column_to_index(column: char) -> usize {
    // Map columns 'a'..='h' to indices 0..=7 by subtracting ASCII codes
    (column as u8 - b'a') as usize
}

pub fn row_to_index(row: char) -> usize {
    // Map rows '8'..='1' to indices 0..=7 by reversing ASCII codes
    (b'8' - row as u8) as usize
}