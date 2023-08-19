//! `Sequence` is designed for use cases in which the elements' order has to get persisted
//! but the chosen persistence layer requires metadata to do so, e.g. a database.
//! `Sequence` allows to add and remove elements at any position, virtually infinitely.

// Re-exports for convenient use within crate.
// none

// Publicly re-exporting all items valuable to users.
// (avoids explicitly listing re-exports in crate documentation as there is no alternate path to those items)
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, AddAssign};
use std::ops::{Index, IndexMut};

use num_integer::gcd;
#[cfg(feature = "serde-derive")]
use serde::{Deserialize, Serialize};

//
// Node
//
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Node<T> {
    position: Pos,
    element: Option<T>,
}

impl<T> Node<T> {
    #[inline]
    #[must_use]
    fn new(position: Pos, element: T) -> Self {
        Node {
            position,
            element: Some(element),
        }
    }

    #[inline]
    #[must_use]
    fn new_empty(position: Pos) -> Self {
        Node { position, element: None }
    }

    #[inline]
    #[must_use]
    fn position(&self) -> Pos {
        self.position
    }

    #[inline]
    #[must_use]
    pub fn pos(&self) -> (u64, u64) {
        (self.position.num, self.position.denom)
    }

    #[inline]
    #[must_use]
    pub fn num(&self) -> u64 {
        self.position.num
    }

    #[inline]
    #[must_use]
    pub fn denom(&self) -> u64 {
        self.position.denom
    }

    #[inline]
    #[must_use]
    pub fn element(self) -> Option<T> {
        self.element
    }

    #[inline]
    #[must_use]
    pub fn element_as_ref(&self) -> Option<&T> {
        self.element.as_ref()
    }

    #[inline]
    #[must_use]
    pub fn element_as_mut(&mut self) -> Option<&mut T> {
        self.element.as_mut()
    }

    #[inline]
    #[must_use]
    pub fn is_none(&self) -> bool {
        self.element.is_none()
    }

    #[inline]
    #[must_use]
    pub fn is_some(&self) -> bool {
        self.element.is_some()
    }

    #[inline]
    fn set(&mut self, element: T) {
        self.element = Some(element)
    }
}

#[cfg(test)]
#[path = "tests/node_tests.rs"]
mod node_tests;

//
// Position
//
const DENOM_MIN: u64 = 1;

/// `Pos` defines the ordering of `Node`s in a `Sequence`.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Pos {
    num: u64,
    denom: u64,
}

trait Min {
    const MIN: Pos;
}

impl Min for Pos {
    const MIN: Pos = Pos { num: u64::MIN, denom: 1 };
}

trait Max {
    const MAX: Pos;
}

impl Max for Pos {
    const MAX: Pos = Pos { num: u64::MAX, denom: 1 };
}

impl Pos {
    /// Creates a valid `Pos`, i.e. the denominator >= 1
    /// If denominator is set to 0, the `Pos` will have a denominator of 1.
    #[inline]
    #[must_use]
    fn new(num: u64, denom: u64) -> Self {
        if denom < DENOM_MIN {
            Pos { num, denom: DENOM_MIN }
        } else {
            Pos { num, denom }
        }
    }

    // Not a valid position, used for incrementing a position only.
    #[inline]
    #[must_use]
    fn n1d0() -> Self {
        Self { num: 1, denom: 0 }
    }

    #[inline]
    #[must_use]
    fn mid(first: Self, second: Self) -> Self {
        first + second
    }
}

/// `Pos` are added to each other by adding their numerators and denominators separately.
/// So, `Pos` does not follow the rules for adding fractions.
impl Add for Pos {
    type Output = Self;

    #[inline]
    #[must_use]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            num: self.num + rhs.num,
            denom: self.denom + rhs.denom,
        }
    }
}

/// `Pos` are added to each other by adding their numerators and denominators separately.
/// So, `Pos` does not follow the rules for adding fractions.
impl AddAssign for Pos {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl PartialEq for Pos {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.num == other.num && self.denom == other.denom {
            true
        } else {
            let self_divisor = gcd(self.num, self.denom);
            let other_divisor = gcd(other.num, other.denom);

            // If greatest common divisor (gcd) is not 1 we have to reduce the
            // Position first by dividing numerator and denominator by the gcd.
            match (self_divisor, other_divisor) {
                (1, 1) => false,
                #[rustfmt::skip]
                (_, 1) => { // marked for tarpaulin
                    let f1 = Self::new(self.num / self_divisor, self.denom / self_divisor);
                    f1 == *other
                }
                #[rustfmt::skip]
                (1, _) => { // marked for tarpaulin
                    let f2 = Self::new(other.num / other_divisor, other.denom / other_divisor);
                    f2 == *self
                }
                #[rustfmt::skip]
                (_, _) => { // marked for tarpaulin
                    let f1 = Self::new(self.num / self_divisor, self.denom / self_divisor);
                    let f2 = Self::new(other.num / other_divisor, other.denom / other_divisor);
                    f1 == f2
                }
            }
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let f1 = self.num as f64 / self.denom as f64;
        let f2 = other.num as f64 / self.denom as f64;

        f1.partial_cmp(&f2)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} ({})", self.num, self.denom, self.num as f64 / self.denom as f64)
    }
}

impl Default for Pos {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::new(1, DENOM_MIN)
    }
}

#[cfg(test)]
#[path = "tests/position_tests.rs"]
mod position_tests;

//
// Sequence
//

/// A `Sequence` with a deterministic and easy to persist ordering.
///
/// What you can do
/// * Create a Sequence: [new()](`Sequence::new()`) or with_capacity()
/// * Determine the capacity: capacity()
/// * Determine if it contains elements: is_empty()
/// * Determine how many elements it contains: len()
/// * Get the first element: first()
/// * Get the last element: last()
/// * Get an immutable reference to an element: get()
/// * Get a mutable reference to an element: get_mut()
/// * Insert an element at a defined index: insert()
/// * Insert an element at a defined positions: insert_at()
/// * Get an element's position from its index: position_from() and pos_from()
/// * Append an element to the sequence: push()
/// * Remove an element at a defined index: remove()
/// * Remove an element at a defined position: remove_at()
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde-derive", derive(Serialize, Deserialize))]
pub struct Sequence<T> {
    nodes: Vec<Node<T>>,
    len: usize,
}

impl<T> Sequence<T> {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            len: 0,
        }
    }

    #[inline]
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    /// Returns the total number of elements the sequence can
    /// hold without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.nodes.capacity()
    }

    /// Returns true if the sequence contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of elements in the sequence, also
    /// referred to as its 'length'.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the first element of the slice, or None
    /// if it is empty.
    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        for node in self.nodes.iter() {
            if node.is_some() {
                return node.element_as_ref();
            }
        }

        #[cfg(not(tarpaulin_include))]
        {
            None
        }
    }

    /// Returns the last element of the slice, or None
    /// if it is empty.
    #[inline]
    #[must_use]
    pub fn last(&self) -> Option<&T> {
        if self.len == 0 {
            return None;
        }

        for node in self.nodes.iter().rev() {
            if node.is_some() {
                return node.element_as_ref();
            }
        }

        #[cfg(not(tarpaulin_include))]
        {
            None
        }
    }

    #[inline]
    #[must_use]
    /// Returns `Some<T>`
    /// Returns None when out of bounds
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        for node in self.nodes[index..].iter() {
            if node.is_some() {
                return node.element_as_ref();
            }
        }

        #[cfg(not(tarpaulin_include))]
        {
            None
        }
    }

    #[inline]
    #[must_use]
    /// Returns `Some<T>`
    /// Returns None when out of bounds
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        for node in self.nodes[index..].iter_mut() {
            if node.is_some() {
                return node.element_as_mut();
            }
        }

        #[cfg(not(tarpaulin_include))]
        {
            None
        }
    }

    /// Iterates the sequence searching for position.
    /// Returns None if position is not in sequence.
    /// Returns Some(index) if position is in sequence.
    ///
    /// # Panics
    /// This function might panic if the iterator has more than usize::MAX non-matching elements.
    // todo: legacy!
    // todo: cover panic!
    pub fn index_from(&self, position: Pos) -> Option<usize> {
        self.nodes.iter().position(|node| node.position() == position)
    }

    /// Inserts an element at index, shifting all elements after it to the right.
    /// Appends the element to the vector if index > len.
    /// Inserts an element into an existing Node or by creating a new Node.
    /// Remember: Index starts at 0.
    ///
    /// if node is empty (i.e. node.element == None) set node.element to Some(element).
    /// if node is not empty (i.e. node.element == Some) insert element into self.nodes.
    ///
    /// # Panics
    /// Unlike `std::vec::Vec`, does not panic.
    pub fn insert(&mut self, index: usize, element: T) {
        if index >= self.nodes.len() {
            // Append element (self.len is incremented by push()).
            self.push(element);
        } else {
            if self.nodes[index].is_some() {
                let pos = if index == 0 {
                    // unwrap() is safe because the prior if covered the self.len() == 0 case.
                    Pos::mid(Pos::MIN, self.nodes.first().map(|node| node.position()).unwrap())
                } else {
                    Pos::mid(
                        self.nodes.get(index - 1).map(|node| node.position()).unwrap(),
                        self.nodes.get(index).map(|node| node.position()).unwrap(),
                    )
                };

                let node = Node::new(pos, element);
                self.nodes.insert(index, node);
            } else {
                self.nodes[index].set(element);
            }

            self.len += 1;
        }
    }

    /// Inserts an element at position. If there is an element at the position, it is overwritten.
    /// If not, element is inserted and all following elements after shifted to the right.
    // todo: use binary_search which required Ord for Position
    pub fn insert_at(&mut self, position: Pos, element: T) {
        match self.index_from(position) {
            None => {
                let index = self.nodes.partition_point(|node| node.position() < position);
                self.insert(index, element);
            }
            Some(index) => {
                // If node does not contain an element, increase len before setting the element.
                if self.nodes[index].is_none() {
                    self.len += 1;
                }

                // Replace the prior element.
                self.nodes[index].set(element);
            }
        }
    }

    // todo: legacy
    pub fn position_from(&self, index: usize) -> Option<Pos> {
        if index >= self.nodes.len() {
            None
        } else {
            Some(self.nodes[index].position())
        }
    }

    #[allow(unused)]
    fn pos_from(&self, index: usize) -> Option<(u64, u64)> {
        if index >= self.nodes.len() {
            None
        } else {
            Some(self.nodes[index].pos())
        }
    }

    /// Appends an element to the back of the sequence.
    #[inline]
    pub fn push(&mut self, element: T) {
        let pos = match self.last_position() {
            None => Pos::new(1, 1),
            Some(pos) => pos + Pos::n1d0(),
        };
        let node = Node::new(pos, element);

        self.nodes.push(node);
        self.len += 1;
    }

    /// Removes and returns the element at index.
    /// Unlike `std::vec::Vec`, does not shift elements after it to the left
    /// because it just replaces the element with None.
    /// So, this has performance of O(1).
    ///
    /// # Panics
    /// Unlike `std::vec::Vec`, does not panic.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        for (i, node) in self.nodes[index..].iter().enumerate() {
            if node.is_some() {
                self.len -= 1;

                // Push an empty node to the end of Vec self.nodes
                // using the position of the node to be removed.
                // Then swap the empty node with the node to remove.
                // Finally, remove the node.
                let node = Node::new_empty(self.nodes[index + i].position());
                self.nodes.push(node);
                return self.nodes.swap_remove(index + i).element();
            }
        }

        #[cfg(not(tarpaulin_include))]
        {
            None
        }
    }

    /// Removes and returns the element at position.
    /// Unlike `std::vec::Vec`, does not shift elements after it to the left
    /// because it just replaces the element with None.
    /// Because we have to search the `Position`, it has performance of O(n/2).
    ///
    /// # Panics
    /// Unlike `std::vec::Vec`, does not panic.
    pub fn remove_at(&mut self, position: Pos) -> Option<T> {
        match self.index_from(position) {
            None => None,
            Some(index) => self.remove(index),
        }
    }

    #[inline]
    #[must_use]
    fn last_position(&self) -> Option<Pos> {
        self.nodes.last().map(|node| node.position())
    }

    #[cfg(not(tarpaulin_include))]
    #[allow(dead_code)]
    fn shrink(&mut self) {
        unimplemented!()
    }
}

impl<T> Default for Sequence<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<usize> for Sequence<T> {
    type Output = Node<T>;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T> IndexMut<usize> for Sequence<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl<T: Clone> Clone for Sequence<T> {
    fn clone(&self) -> Self {
        let mut seq: Sequence<T> = Sequence::new();

        for node in self {
            let node = match node.element_as_ref() {
                None => Node::new_empty(node.position()),
                Some(element) => Node::new(node.position(), element.clone()),
            };
            seq.nodes.push(node);
        }
        seq.len = self.len;
        seq
    }
}

//
// Consuming Iterator
//

// IntoIterator for Sequence<T>
// - consumes the `Sequence`
// - removes all empty Nodes from `Sequence`
// - iterates over remaining Nodes
impl<T> IntoIterator for Sequence<T> {
    type Item = Node<T>;
    type IntoIter = std::vec::IntoIter<Node<T>>;

    #[inline]
    fn into_iter(mut self) -> Self::IntoIter {
        self.nodes.retain(|node| node.element_as_ref().is_some());
        self.nodes.into_iter()
    }
}

//
// Immutable Iterator
//

// Iterator / IntoIterator over a `Sequence` represented by a slice of immutable `Option<&Node>`
// - allows to use a Sequence in a for loop
pub struct SequenceIterator<'iterator, T: 'iterator>(Option<&'iterator [Node<T>]>);

impl<'iterator, T: 'iterator> Iterator for SequenceIterator<'iterator, T> {
    type Item = &'iterator Node<T>;

    fn next(self: &'_ mut SequenceIterator<'iterator, T>) -> Option<Self::Item> {
        self.0.and_then(|v| {
            let (head, tail) = v.split_first()?;
            self.0 = Some(tail);
            Some(head)
        })
    }
}

impl<'iterator, T: 'iterator> IntoIterator for &'iterator Sequence<T> {
    type Item = &'iterator Node<T>;
    type IntoIter = SequenceIterator<'iterator, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SequenceIterator(Some(self.nodes.as_slice()))
    }
}

//
// Mutable Iterator
//

// Iterator / IntoIterator over a `Sequence` represented by a slice of mutable `Option<&mut Node>`
// - allows to use a Sequence in a for loop
pub struct SequenceIteratorMut<'iterator, T: 'iterator>(Option<&'iterator mut [Node<T>]>);

impl<'iterator, T: 'iterator> Iterator for SequenceIteratorMut<'iterator, T> {
    type Item = &'iterator mut Node<T>;

    fn next(self: &'_ mut SequenceIteratorMut<'iterator, T>) -> Option<Self::Item> {
        self.0.take().and_then(|v| {
            let (head, tail) = v.split_first_mut()?;
            self.0 = Some(tail);
            Some(head)
        })
    }
}

impl<'iterator, T: 'iterator> IntoIterator for &'iterator mut Sequence<T> {
    type Item = &'iterator mut Node<T>;
    type IntoIter = SequenceIteratorMut<'iterator, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SequenceIteratorMut(Some(self.nodes.as_mut_slice()))
    }
}

#[cfg(test)]
#[path = "tests/sequence_tests.rs"]
mod sequence_tests;
