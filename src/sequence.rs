#[cfg(test)]
#[path = "tests/sequence/tests.rs"]
mod tests;

use crate::position::Min;
use crate::Position;
use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq)]
pub struct Sequence<T> {
    positions: Vec<Position>,
    elements: Vec<T>,
    current_index: usize,
}

impl<T> Sequence<T> {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            elements: vec![],
            current_index: 0,
        }
    }

    pub fn first(&self) -> Option<&T> {
        self.elements.first()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    pub fn last(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    // todo: use binary_search which required Ord for Position
    pub fn index_from(&self, position: Position) -> Option<usize> {
        self.positions.iter().position(|&p| p == position)
    }

    /// Inserts an element at index, shifting all elements after it to the right.
    /// Appends the element to the vector if index > len.
    /// Remember: Index starts at 0.
    pub fn insert(&mut self, index: usize, element: T) {
        if index >= self.len() {
            // Append element.
            self.push(element)
        } else if index == 0 {
            // Prepend element by finding the position between MIN and first element.
            let pos = Position::mid(Position::MIN, *self.positions.first().unwrap());
            self.positions.insert(index, pos);
            self.elements.insert(index, element);
        } else {
            // Insert element between two elements.
            let pos = Position::mid(*self.positions.get(index - 1).unwrap(), *self.positions.get(index).unwrap());
            self.positions.insert(index, pos);
            self.elements.insert(index, element);
        }
    }

    /// Inserts an element at position. If there is an element with the same position, it is overwritten
    /// shifting all elements after it to the right.
    // todo: use binary_search which required Ord for Position
    pub fn insert_at(&mut self, position: Position, element: T) {
        match self.index_from(position) {
            Some(index) => {
                self.elements[index] = element;
            }
            None => {
                let index = self.positions.partition_point(|&p| p < position);
                self.insert(index, element);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Run `non_iterating_next` until None is returned
    pub fn non_iterating_next(&mut self) -> Option<(u64, u64, &T)> {
        if self.current_index < self.elements.len() {
            let (n, d, element) = (
                self.positions[self.current_index].numerator,
                self.positions[self.current_index].denominator,
                &self.elements[self.current_index],
            );
            self.current_index += 1;
            Some((n, d, element))
        } else {
            self.current_index = 0;
            None
        }
    }

    pub fn position_from(&self, index: usize) -> Position {
        self.positions[index]
    }

    /// Appends an element to the back of the sequence.
    pub fn push(&mut self, element: T) {
        let pos = match self.positions.last().cloned() {
            None => Position::new(1, 1),
            Some(mut pos) => {
                pos += Position::n1d0();
                pos
            }
        };

        self.positions.push(pos);
        self.elements.push(element);
    }

    // Removes and returns the element at index, shifting all elements after it to the left.
    // Note: Because this shifts the remaining elements, it has a worst-case performance of O(n).
    // Does not panic!
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.elements.len() {
            self.positions.remove(index);
            Some(self.elements.remove(index))
        } else {
            None
        }
    }

    // Removes and returns the element at position, shifting all elements after it to the left.
    // Note: Because this shifts the remaining elements, it has a worst-case performance of O(n).
    // Does not panic!
    pub fn remove_at(&mut self, position: Position) -> Option<T> {
        match self.index_from(position) {
            None => None,
            Some(index) => self.remove(index),
        }
    }
}

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Sequence<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for Sequence<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

/*
impl<T> Iterator for Sequence<T> {
    type Item = (u64, u64, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.elements.len() {
            let (n, d, element) = (self.positions[self.current_index].numerator,
                                       self.positions[self.current_index].denominator,
                                       &self.elements[self.current_index]);
            self.current_index += 1;
            Some((n, d, element))
        } else {
            None
        }
    }
}
*/
