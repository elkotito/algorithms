//! Scala style mutable linked list.
//! Not very useful in Rust, but a good exercise to learn Box<T>.
//! Recursive implementation was for an educational purpose only.

#[derive(Debug, Default)]
pub enum ConsList<T> {
    Cons(T, Box<ConsList<T>>),
    #[default]
    Nil,
}

impl<T> ConsList<T> {
    pub fn new() -> Self {
        Self::Nil
    }

    pub fn append(&mut self, item: T) {
        match self {
            Self::Cons(_, next) => next.append(item),
            Self::Nil => {
                *self = Self::Cons(item, Box::new(Self::Nil));
            }
        }
    }

    pub fn prepend(&mut self, item: T) {
        let old = std::mem::replace(self, Self::Nil);
        *self = Self::Cons(item, Box::new(old));
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match (self, index) {
            (Self::Cons(value, _), 0) => Some(value),
            (Self::Cons(_, next), n) if n != 0 => next.get(n - 1),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match (self, index) {
            (Self::Cons(value, _), 0) => Some(value),
            (Self::Cons(_, next), n) => next.get_mut(n - 1),
            _ => None,
        }
    }
}

pub struct ConsListIter<'a, T> {
    list: &'a ConsList<T>,
}

impl<'a, T> Iterator for ConsListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list {
            ConsList::Cons(value, next) => {
                self.list = next;
                Some(value)
            }
            ConsList::Nil => None,
        }
    }
}

// `next` moves this mutable borrow for `value` out to return `&'a mut T`.
// `Option::take` leaves `None` behind until we store the next node.
pub struct ConsListMutIter<'a, T> {
    list: Option<&'a mut ConsList<T>>,
}

impl<'a, T> Iterator for ConsListMutIter<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.list.take()? {
            ConsList::Cons(value, next) => {
                self.list = Some(next);
                Some(value)
            }
            ConsList::Nil => None,
        }
    }
}

impl<T> ConsList<T> {
    pub fn iter(&self) -> ConsListIter<'_, T> {
        ConsListIter { list: self }
    }

    pub fn iter_mut(&mut self) -> ConsListMutIter<'_, T> {
        ConsListMutIter { list: Some(self) }
    }
}

// Required to implement for `collect()`. Useful for testing.
impl<T> FromIterator<T> for ConsList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = ConsList::new();
        for item in iter {
            list.prepend(item);
        }

        list
    }
}
