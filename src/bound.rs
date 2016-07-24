// The MIT License (MIT)
// 
// Copyright (c) 2016 Skylor R. Schermer
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in 
// all copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
//!
//! Provides a basic bounded interval type for doing complex set selections.
//!
////////////////////////////////////////////////////////////////////////////////
// Module imports.
use std::default::Default;


////////////////////////////////////////////////////////////////////////////////
// Bound<T>
////////////////////////////////////////////////////////////////////////////////
/// Determines the type of an interval's boundary.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Bound<T> {
    /// The boundary includes the point.
    Included(T),
    /// The boundary excludes the point.
    Excluded(T),
}

impl<T> Bound<T> where T: PartialOrd + PartialEq + Clone {
    /// Returns the point marking at the bound.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(1);
    /// 
    /// assert_eq!(b1.point(), &0);
    /// assert_eq!(b2.point(), &1);
    /// ```
    #[inline]
    pub fn point(&self) -> &T {
        match *self {
            Bound::Included(ref bound) => bound,
            Bound::Excluded(ref bound) => bound
        }
    }

    /// Returns whether the boundary includes its point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(1);
    /// 
    /// assert!(b1.is_closed());
    /// assert!(!b2.is_closed());
    /// ```
    #[inline]
    pub fn is_closed(&self) -> bool {
        match *self {
            Bound::Included(..) => true,
            Bound::Excluded(..) => false
        }
    }

    /// Returns whether the boundary excludes its point. 
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(1);
    /// 
    /// assert!(!b1.is_open());
    /// assert!(b2.is_open());
    /// ```
    #[inline]
    pub fn is_open(&self) -> bool {
        !self.is_closed()
    }

    /// Returns the intersect of the given boundaries, or the lowest one if they
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.intersect_or_least(&b2), b2);
    /// ```
    #[inline]
    pub fn intersect_or_least(&self, other: &Self) -> Self {
        if self.point() == other.point() {
            if self.is_closed() && other.is_closed() {
                self.clone()
            } else {
                Bound::Excluded(self.point().clone())
            }
        } else if self.point() < other.point() {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the intersect of the given boundaries, or the greatest one if 
    /// they are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.intersect_or_greatest(&b2), b2);
    /// ```
    #[inline]
    pub fn intersect_or_greatest(&self, other: &Self) -> Self {
        if self.point() == other.point() {
            if self.is_closed() && other.is_closed() {
                self.clone()
            } else {
                Bound::Excluded(self.point().clone())
            }
        } else if self.point() > other.point() {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the union of the given boundaries, or the lowest one if they are
    /// not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.union_or_least(&b2), b1);
    /// ```
    #[inline]
    pub fn union_or_least(&self, other: &Self) -> Self {
        if self.point() == other.point() {
            if self.is_open() && other.is_open() {
                self.clone()
            } else {
                Bound::Included(self.point().clone())
            }
        } else if self.point() < other.point() {
            self.clone()
        } else {
            other.clone()
        }
    }

    /// Returns the union of the given boundaries, or the greatest one if they 
    /// are not at the same point.
    ///
    /// # Example
    ///
    /// ```rust
    /// use interval::Bound;
    ///
    /// let b1 = Bound::Included(0);
    /// let b2 = Bound::Excluded(0);
    /// 
    /// assert_eq!(b1.union_or_greatest(&b2), b1);
    /// ```
    #[inline]
    pub fn union_or_greatest(&self, other: &Self) -> Self {
        if self.point() == other.point() {
            if self.is_open() && other.is_open() {
                self.clone()
            } else {
                Bound::Included(self.point().clone())
            }
        } else if self.point() > other.point() {
            self.clone()
        } else {
            other.clone()
        }
    }
}

// Default bound is closed.
impl<T> Default for Bound<T> where T: Default {
    fn default() -> Self {
        Bound::Included(Default::default())
    }
}

// Bound-from-Point conversion.
impl<T> From<T> for Bound<T> {
    fn from(t: T) -> Self {
        Bound::Included(t)
    }
}
