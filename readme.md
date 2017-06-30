
interval-rs
===========

A normalizing interval library.


The normalization API of `Interval` provides three major features:

    + Closure: An `Interval` will automatically have its bounds rewritten to conform to finite or countable type specifications.

    + Iteration: An `Interval` can alternate between normalized and denormalized forms to provide left and right iterators over its interior.

    + Widening: An `Interval` can be unioned with with an interval with different boundary points if the closure of those boundaries would overlap. 

The closure and unification features are provided by the Normalize trait, which is blanket implemented for all Intervals. We use specialization to override the default 'do nothing' behavior.

The iteration feature is provided by the LeftIterable and RightIterable trait. These traits are implemented on the iterator's elements and used to treat the intervals as consuming iterators.

The widening feature is also provided by the LeftIterable and RightIterable traits, but through the NextUpper and NextLower traits, respectively.



Normalization Traits
====================


LeftIterable: Clone
RightIterable: Clone

Normalize: Sized

NextLower: Sized
NextUpper: Sized

