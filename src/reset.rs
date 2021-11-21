use std::ops::{Deref, DerefMut};
use crate::Modified;

/// Modified value that allows to be reseted.
pub struct Resetable<T>(pub(crate) Modified<T>);

impl<T> Resetable<T> {
    /// Creates new [`Resetable`] with `v` inside.
    /// ```
    /// # use modified::Resetable;
    /// let m = Resetable::new(15);
    /// assert_eq!(*m, 15);
    /// ```
    #[inline]
    pub fn new(v: T) -> Self {
        Self(Modified::new(v))
    }

    /// Creates new [`Resetable`] with `v` inside and marks it as it was modified.
    /// ```
    /// # use modified::Resetable;
    /// let m = Resetable::new_modified(15);
    /// assert!(m.is_modified());
    /// ```
    #[inline]
    pub fn new_modified(v: T) -> Self {
        Self(Modified::new_modified(v))
    }

    /// Resets internal state.
    /// If value was marked as modified it no longer is!
    /// ```
    /// # use modified::{Resetable, Modified};
    /// let mut r = Resetable::new(-15);
    /// *r = 20;
    /// assert!(r.is_modified());
    /// 
    /// r.reset();
    /// // No longer modified!
    /// 
    /// assert!(r.is_unchanged());
    /// ```
    #[inline]
    pub fn reset(&mut self) {
        self.0.1 = false;
    }
    
    /// Destroys previous valus inside [`Resetable`] replacing it with the new one.
    /// ```
    /// # use modified::Resetable;
    /// let mut m = Resetable::new(15);
    /// m.set(20);
    /// assert_eq!(*m, 20);
    /// ```
    #[inline]
    pub fn set(&mut self, v: T) {
        **self = v;
    }

    /// Returns a reference to the inner value.
    /// ```
    /// # use modified::Resetable;
    /// let m = Resetable::new(15);
    /// assert_eq!(m.get(), &15);
    /// ```
    #[inline]
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Returns a reference to the inner value and its state.
    /// If value was modified state will be `true` otherwise, `false`.
    /// ```
    /// # use modified::Resetable;
    /// let m = Resetable::new(15);
    /// assert_eq!(m.get_value_changed(), (&15, false));
    /// ```
    #[inline]
    pub fn get_value_changed(&self) -> (&T, bool) {
        (&self.0, self.0.1)
    }

    /// Returns the ownership of the inner value.
    /// ```
    /// # use modified::Resetable;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Owned(i32);
    /// let m = Resetable::new(Owned(15));
    /// assert_eq!(m.into_inner(), Owned(15));
    /// ```
    #[inline]
    pub fn into_inner(self) -> T {
        self.0.0
    }

    /// Returns the ownership of the inner value and its state.
    /// If value was modified state will be `true` otherwise, `false`.
    /// ```
    /// # use modified::Resetable;
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Owned(i32);
    /// let m = Resetable::new(Owned(15));
    /// assert_eq!(m.into_inner_changed(), (Owned(15), false));
    /// ```
    #[inline]
    pub fn into_inner_changed(self) -> (T, bool) {
        (self.0.0, self.0.1)
    }

    /// Returns `true` if the variable inside was modified, otherwise returns `false`.
    /// ```
    /// # use modified::Resetable;
    /// let mut m = Resetable::new(15);
    /// *m = 20;
    /// assert!(m.is_modified());
    /// ```
    #[inline]
    pub fn is_modified(&self) -> bool {
        self.0.1
    }

    /// Returns `true` if the variable inside wasn't changed, otherwise returns `false`.
    /// ```
    /// # use modified::Resetable;
    /// let mut m = Resetable::new(15);
    /// *m = 20;
    /// assert!(!m.is_unchanged());
    #[inline]
    pub fn is_unchanged(&self) -> bool {
        !self.0.1
    }
}


impl<T> Default for Resetable<T>
where
    T: Default
{
    /// Creates new Modified from default value of `T`.
    #[inline]
    fn default() -> Self {
        Self(Modified::default())
    }
}

impl<T> Resetable<T>
where
    T: Default
{
    /// Creates new [`Modified`] from the default value of `T` and marks it as it was modified.
    #[inline]
    pub fn default_modified() -> Self {
        Self(Modified::default_modified())
    }
}

impl<T> Clone for Resetable<T>
where
    T: Clone
{
    /// Clones inner value with it's state.
    /// That means that if value was changed, cloned will also be marked as changed.
    #[inline]
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Deref for Resetable<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> DerefMut for Resetable<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}