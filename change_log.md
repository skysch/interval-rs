
# Releases

The following is a change log, documenting added, removed, changed, depricated, and fixed features, along with the version number and official release date of that version.

## Unreleased
-------------

Implemented changes not yet published.

### Added
+ N/A

### Fixed


## normalize_interval 0.14.0  [2020-07-18]
----------------------------------------

### Changed
+ `Selection<T>` now iterates over points by default; interval iterators are given dedicated methods.
+ `TineTree::iter_intervals` renamed to `InteTree::interval_iter` to match the rest of the library.

## normalize_interval 0.13.1  [2020-07-18]
----------------------------------------

### Added
+ `FromIterator<T>` impl for `Selection<T>`.


## normalize_interval 0.13.0  [2020-07-18]
----------------------------------------

### Added
+ Initial release.

### Removed
+ Non-finite interval support.
