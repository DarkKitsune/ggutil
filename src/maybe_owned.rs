use std::{fmt::Debug, hash::{Hash, Hasher}};

/// A value that is either owned or borrowed.
pub enum MaybeOwned<'a, T: 'a> {
    Owned(T),
    Borrowed(&'a T),
    BorrowedMut(&'a mut T),
}

impl<'a, T: 'a> MaybeOwned<'a, T> {
    /// Returns whether the value is owned.
    pub fn is_owned(&self) -> bool {
        match self {
            MaybeOwned::Owned(_) => true,
            MaybeOwned::Borrowed(_) => false,
            MaybeOwned::BorrowedMut(_) => false,
        }
    }

    /// Returns whether the value is borrowed.
    pub fn is_borrowed(&self) -> bool {
        !self.is_owned()
    }

    /// Unwraps and returns a `Some` value containing the inner value if it is owned, or `None` if it is borrowed.
    pub fn unwrap(self) -> Option<T> {
        match self {
            MaybeOwned::Owned(value) => Some(value),
            MaybeOwned::Borrowed(_) => None,
            MaybeOwned::BorrowedMut(_) => None,
        }
    }

    /// Unwraps and returns the inner value. If the value is borrowed, it is cloned.
    pub fn unwrap_or_clone(self) -> T
    where T: Clone,
    {
        match self {
            MaybeOwned::Owned(value) => value,
            MaybeOwned::Borrowed(value) => value.clone(),
            MaybeOwned::BorrowedMut(value) => value.clone(),
        }
    }

    /// Forces the value to be owned. Clones the value if it is borrowed.
    pub fn take_ownership(&mut self)
        where T: Clone,
    {
        match self {
            MaybeOwned::Owned(_) => {}
            MaybeOwned::Borrowed(value) => *self = MaybeOwned::Owned(value.clone()),
            MaybeOwned::BorrowedMut(value) => *self = MaybeOwned::Owned(value.clone()),
        }
    }

    /// Borrows the inner value
    pub fn as_ref(&self) -> &T {
        match self {
            MaybeOwned::Owned(value) => value,
            MaybeOwned::Borrowed(value) => value,
            MaybeOwned::BorrowedMut(value) => value,
        }
    }

    /// Borrows the inner value mutably if the value is mutable, otherwise returns `None`.
    pub fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            MaybeOwned::Owned(value) => Some(value),
            MaybeOwned::Borrowed(_) => None,
            MaybeOwned::BorrowedMut(value) => Some(value),
        }
    }
}

impl<'a, T: 'a> From<T> for MaybeOwned<'a, T> {
    fn from(value: T) -> Self {
        MaybeOwned::Owned(value)
    }
}

impl<'a, T: 'a> From<&'a T> for MaybeOwned<'a, T> {
    fn from(value: &'a T) -> Self {
        MaybeOwned::Borrowed(value)
    }
}

impl<'a, T: 'a> From<&'a mut T> for MaybeOwned<'a, T> {
    fn from(value: &'a mut T) -> Self {
        MaybeOwned::BorrowedMut(value)
    }
}

impl<'a, T: 'a + Clone> Clone for MaybeOwned<'a, T> {
    fn clone(&self) -> Self {
        match self {
            MaybeOwned::Owned(value) => MaybeOwned::Owned(value.clone()),
            MaybeOwned::Borrowed(value) => MaybeOwned::Borrowed(value),
            MaybeOwned::BorrowedMut(value) => MaybeOwned::Owned((*value).clone()),
        }
    }
}

impl<'a, T: 'a + Debug> Debug for MaybeOwned<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MaybeOwned::Owned(value) => Debug::fmt(value, f),
            MaybeOwned::Borrowed(value) => Debug::fmt(value, f),
            MaybeOwned::BorrowedMut(value) => Debug::fmt(value, f),
        }
    }
}

impl<'a, T: 'a + PartialEq> PartialEq for MaybeOwned<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            MaybeOwned::Owned(value) => match other {
                MaybeOwned::Owned(other_value) => value == other_value,
                MaybeOwned::Borrowed(other_value) => value == *other_value,
                MaybeOwned::BorrowedMut(other_value) => value == *other_value,
            },
            MaybeOwned::Borrowed(value) => match other {
                MaybeOwned::Owned(other_value) => *value == other_value,
                MaybeOwned::Borrowed(other_value) => *value == *other_value,
                MaybeOwned::BorrowedMut(other_value) => *value == *other_value,
            },
            MaybeOwned::BorrowedMut(value) => match other {
                MaybeOwned::Owned(other_value) => *value == other_value,
                MaybeOwned::Borrowed(other_value) => *value == *other_value,
                MaybeOwned::BorrowedMut(other_value) => *value == *other_value,
            },
        }
    }
}

impl<'a, T: 'a + Eq> Eq for MaybeOwned<'a, T> {}

impl<'a, T: 'a + Hash> Hash for MaybeOwned<'a, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            MaybeOwned::Owned(value) => value.hash(state),
            MaybeOwned::Borrowed(value) => value.hash(state),
            MaybeOwned::BorrowedMut(value) => value.hash(state),
        }
    }
}

impl<'a, T: 'a> std::ops::Deref for MaybeOwned<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            MaybeOwned::Owned(value) => value,
            MaybeOwned::Borrowed(value) => value,
            MaybeOwned::BorrowedMut(value) => value,
        }
    }
}

impl<'a, T: 'a> AsRef<T> for MaybeOwned<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            MaybeOwned::Owned(value) => value,
            MaybeOwned::Borrowed(value) => value,
            MaybeOwned::BorrowedMut(value) => value,
        }
    }
}