use std::ops::{Deref, DerefMut};
use crate::Resetable;

/// Struct that holds value and tracks if it was modified.
pub struct Modified<T>(pub(crate) T, pub(crate) bool);

impl<T> Modified<T> {
    /// Creates new [`Modified`] with `v` inside.
    /// ```
    /// # use modified::Modified;
    /// let m = Modified::new(15);
    /// assert_eq!(*m, 15);
    /// ```
    #[inline]
    pub fn new(v: T) -> Self {
        Self(v, false)
    }

    /// Creates new [`Modified`] with `v` inside and marks it as it was modified.
    /// ```
    /// # use modified::Modified;
    /// let m = Modified::new_modified(15);
    /// assert!(m.is_modified());
    /// ```
    #[inline]
    pub fn new_modified(v: T) -> Self {
        Self(v, true)
    }

    /// Destroys previous valus inside [`Modified`] replacing it with the new one.
    /// ```
    /// # use modified::Modified;
    /// let mut m = Modified::new(15);
    /// m.set(20);
    /// assert_eq!(*m, 20);
    /// ```
    #[inline]
    pub fn set(&mut self, v: T) {
        **self = v;
    }

    /// Returns a reference to the inner value.
    /// ```
    /// # use modified::Modified;
    /// let m = Modified::new(15);
    /// assert_eq!(m.get(), &15);
    /// ```
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Returns a reference to the inner value and its state.
    /// If value was modified state will be `true` otherwise, `false`.
    /// ```
    /// # use modified::Modified;
    /// let m = Modified::new(15);
    /// assert_eq!(m.get_value_changed(), (&15, false));
    /// ```
    #[inline]
    pub fn get_value_changed(&self) -> (&T, bool) {
        (&self.0, self.1)
    }

    /// Returns the ownership of the inner value.
    /// ```
    /// # use modified::Modified;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Owned(i32);
    /// let m = Modified::new(Owned(15));
    /// assert_eq!(m.into_inner(), Owned(15));
    /// ```
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Returns the ownership of the inner value and its state.
    /// If value was modified state will be `true` otherwise, `false`.
    /// ```
    /// # use modified::Modified;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Owned(i32);
    /// let m = Modified::new(Owned(15));
    /// assert_eq!(m.into_inner_changed(), (Owned(15), false));
    /// ```
    #[inline]
    pub fn into_inner_changed(self) -> (T, bool) {
        (self.0, self.1)
    }

    /// Returns `true` if the variable inside was modified, otherwise returns `false`.
    /// ```
    /// # use modified::Modified;
    /// let mut m = Modified::new(15);
    /// *m = 20;
    /// assert!(m.is_modified());
    /// ```
    #[inline]
    pub fn is_modified(&self) -> bool {
        self.1
    }

    /// Returns `true` if the variable inside wasn't changed, otherwise returns `false`.
    /// ```
    /// # use modified::Modified;
    /// let mut m = Modified::new(15);
    /// *m = 20;
    /// assert!(!m.is_unchanged());
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        !self.1
    }
}

impl<T> Default for Modified<T>
where
    T: Default
{
    /// Creates new Modified from default value of `T`.
    #[inline]
    fn default() -> Self {
        Self(T::default(), false)
    }
}

impl<T> Modified<T>
where
    T: Default
{
    /// Creates new [`Modified`] from the default value of `T` and marks it as it was modified.
    #[inline]
    pub fn default_modified() -> Self {
        Self(T::default(), true)
    }
}

impl<T> Clone for Modified<T>
where
    T: Clone
{
    /// Clones inner value with it's state.
    /// That means that if value was changed, cloned will also be marked as changed.
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T> Into<Resetable<T>> for Modified<T> {
    #[inline]
    fn into(self) -> Resetable<T> {
        Resetable(self)
    }
}

impl<T> Deref for Modified<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Modified<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1 = true;
        &mut self.0
    }
}
