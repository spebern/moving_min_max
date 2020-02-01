//! Keep track of the minimum or maximum value in a sliding window.
//!
//! `moving min max` provides one data structure for keeping track of the
//! minimum value and one for keeping track of the maximum value in a sliding
//! window.
//!
//! The algorithm is based on the description [here](https://stackoverflow.com/questions/4802038/implement-a-queue-in-which-push-rear-pop-front-and-get-min-are-all-consta).
//!
//! The complexity of the operations are
//! - O(1) for getting the minimum/maximum
//! - O(1) for push
//! - amortized O(1) for pop
//!
//! ```rust
//! use moving_min_max::MovingMin;
//!
//! let mut moving_min = MovingMin::<i32>::new();
//! moving_min.push(2);
//! moving_min.push(1);
//! moving_min.push(3);
//!
//! assert_eq!(moving_min.min(), Some(&1));
//! assert_eq!(moving_min.pop(), Some(2));
//!
//! assert_eq!(moving_min.min(), Some(&1));
//! assert_eq!(moving_min.pop(), Some(1));
//!
//! assert_eq!(moving_min.min(), Some(&3));
//! assert_eq!(moving_min.pop(), Some(3));
//!
//! assert_eq!(moving_min.min(), None);
//! assert_eq!(moving_min.pop(), None);
//! ```
//!
//! or
//!
//! ```rust
//! use moving_min_max::MovingMax;
//!
//! let mut moving_max = MovingMax::<i32>::new();
//! moving_max.push(2);
//! moving_max.push(3);
//! moving_max.push(1);
//!
//! assert_eq!(moving_max.max(), Some(&3));
//! assert_eq!(moving_max.pop(), Some(2));
//!
//! assert_eq!(moving_max.max(), Some(&3));
//! assert_eq!(moving_max.pop(), Some(3));
//!
//! assert_eq!(moving_max.max(), Some(&1));
//! assert_eq!(moving_max.pop(), Some(1));
//!
//! assert_eq!(moving_max.max(), None);
//! assert_eq!(moving_max.pop(), None);
//! ```

/// `MovingMin` provides O(1) access to the minimum of a sliding window.
pub struct MovingMin<T> {
    push_stack: Vec<(T, T)>,
    pop_stack: Vec<(T, T)>,
}

impl<T: Clone + PartialOrd> MovingMin<T> {
    /// Creates a new `MovingMin` to keep track of the minimum in a sliding
    /// window.
    pub fn new() -> Self {
        Self {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    /// Creates a new `MovingMin` to keep track of the minimum in a sliding
    /// window with `capacity` allocated slots.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            push_stack: Vec::with_capacity(capacity),
            pop_stack: Vec::with_capacity(capacity),
        }
    }

    /// Returns the minimum of the sliding window or `None` if the window is
    /// empty.
    pub fn min(&self) -> Option<&T> {
        match (self.push_stack.last(), self.pop_stack.last()) {
            (None, None) => None,
            (Some((_, min)), None) => Some(min),
            (None, Some((_, min))) => Some(min),
            (Some((_, a)), Some((_, b))) => Some(if a < b { a } else { b }),
        }
    }

    /// Pushes a new element into the sliding window.
    pub fn push(&mut self, val: T) {
        self.push_stack.push(match self.push_stack.last() {
            Some((_, min)) => {
                if val > *min {
                    (val, min.clone())
                } else {
                    (val.clone(), val)
                }
            }
            None => (val.clone(), val),
        });
    }

    /// Removes and returns the last value of the sliding window.
    pub fn pop(&mut self) -> Option<T> {
        if self.pop_stack.is_empty() {
            match self.push_stack.pop() {
                Some((val, _)) => {
                    self.pop_stack.push((val.clone(), val));
                    while let Some((val, _)) = self.push_stack.pop() {
                        // This is save, because we just pushed one element onto
                        // pop_stack and therefore it cannot be empty.
                        let last =
                            unsafe { self.pop_stack.get_unchecked(self.pop_stack.len() - 1) };
                        let min = if last.1 < val {
                            last.1.clone()
                        } else {
                            val.clone()
                        };
                        self.pop_stack.push((val.clone(), min));
                    }
                }
                None => return None,
            }
        }
        self.pop_stack.pop().map(|(val, _)| val)
    }

    /// Returns the number of elements stored in the sliding window.
    pub fn len(&self) -> usize {
        self.push_stack.len() + self.pop_stack.len()
    }
}

/// `MovingMax` provides O(1) access to the maximum of a sliding window.
pub struct MovingMax<T> {
    push_stack: Vec<(T, T)>,
    pop_stack: Vec<(T, T)>,
}

impl<T: Clone + PartialOrd> MovingMax<T> {
    /// Creates a new `MovingMax` to keep track of the maximum in a sliding window.
    pub fn new() -> Self {
        Self {
            push_stack: Vec::new(),
            pop_stack: Vec::new(),
        }
    }

    /// Creates a new `MovingMax` to keep track of the maximum in a sliding window with
    /// `capacity` allocated slots.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            push_stack: Vec::with_capacity(capacity),
            pop_stack: Vec::with_capacity(capacity),
        }
    }

    /// Returns the maximum of the sliding window or `None` if the window is empty.
    pub fn max(&self) -> Option<&T> {
        match (self.push_stack.last(), self.pop_stack.last()) {
            (None, None) => None,
            (Some((_, max)), None) => Some(max),
            (None, Some((_, max))) => Some(max),
            (Some((_, a)), Some((_, b))) => Some(if a > b { a } else { b }),
        }
    }

    /// Pushes a new element into the sliding window.
    pub fn push(&mut self, val: T) {
        self.push_stack.push(match self.push_stack.last() {
            Some((_, max)) => {
                if val < *max {
                    (val, max.clone())
                } else {
                    (val.clone(), val)
                }
            }
            None => (val.clone(), val),
        });
    }

    /// Removes and returns the last value of the sliding window.
    pub fn pop(&mut self) -> Option<T> {
        if self.pop_stack.is_empty() {
            match self.push_stack.pop() {
                Some((val, _)) => {
                    self.pop_stack.push((val.clone(), val));
                    while let Some((val, _)) = self.push_stack.pop() {
                        // This is save, because we just pushed one element onto
                        // pop_stack and therefore it cannot be empty.
                        let last =
                            unsafe { self.pop_stack.get_unchecked(self.pop_stack.len() - 1) };
                        let max = if last.1 > val {
                            last.1.clone()
                        } else {
                            val.clone()
                        };
                        self.pop_stack.push((val.clone(), max));
                    }
                }
                None => return None,
            }
        }
        self.pop_stack.pop().map(|(val, _)| val)
    }

    /// Returns the number of elements stored in the sliding window.
    pub fn len(&self) -> usize {
        self.push_stack.len() + self.pop_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_min() {
        let mut moving_min = MovingMin::<i32>::new();
        moving_min.push(1);
        assert_eq!(moving_min.min(), Some(&1));
        moving_min.push(2);
        assert_eq!(moving_min.min(), Some(&1));
        moving_min.push(3);
        assert_eq!(moving_min.min(), Some(&1));
        assert_eq!(moving_min.pop(), Some(1));
        assert_eq!(moving_min.min(), Some(&2));
        assert_eq!(moving_min.pop(), Some(2));
        assert_eq!(moving_min.pop(), Some(3));
        assert_eq!(moving_min.pop(), None);

        moving_min.push(2);
        moving_min.push(1);
        moving_min.push(3);
        assert_eq!(moving_min.min(), Some(&1));
        assert_eq!(moving_min.pop(), Some(2));
        assert_eq!(moving_min.min(), Some(&1));
        assert_eq!(moving_min.pop(), Some(1));
        assert_eq!(moving_min.min(), Some(&3));
        assert_eq!(moving_min.pop(), Some(3));
        assert_eq!(moving_min.min(), None);
        assert_eq!(moving_min.pop(), None);
    }

    #[test]
    fn moving_max() {
        let mut moving_max = MovingMax::<i32>::new();
        moving_max.push(3);
        assert_eq!(moving_max.max(), Some(&3));
        moving_max.push(2);
        assert_eq!(moving_max.max(), Some(&3));
        moving_max.push(1);
        assert_eq!(moving_max.max(), Some(&3));
        assert_eq!(moving_max.pop(), Some(3));
        assert_eq!(moving_max.max(), Some(&2));
        assert_eq!(moving_max.pop(), Some(2));
        assert_eq!(moving_max.pop(), Some(1));
        assert_eq!(moving_max.pop(), None);

        moving_max.push(2);
        moving_max.push(3);
        moving_max.push(1);
        assert_eq!(moving_max.max(), Some(&3));
        assert_eq!(moving_max.pop(), Some(2));
        assert_eq!(moving_max.max(), Some(&3));
        assert_eq!(moving_max.pop(), Some(3));
        assert_eq!(moving_max.max(), Some(&1));
        assert_eq!(moving_max.pop(), Some(1));
        assert_eq!(moving_max.max(), None);
        assert_eq!(moving_max.pop(), None);
    }
}
