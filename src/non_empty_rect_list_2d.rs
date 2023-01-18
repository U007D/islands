use std::{fmt::Debug, num::NonZeroUsize};

use crate::error::non_empty_rect_list_2d::{Error, Result};

/// Represents a non-empty, contiguous, rectangular (all rows are the same length) 2D list of items.
/// Despite the internal representation being a 1D `Box`ed (heap-allocated) slice,
/// for convenience and to help ensure correctness, [`NonEmptyRectList2D`] presents a 2D
/// (row, column) interface to the user.
///
/// # Returns:
/// `Err(Error::NoData(String))`: when `rows == 0` or `cols == 0`.
/// `Err(Error::TooManyElements(rows, cols))`: when `rows * cols > usize::MAX`.
/// `Ok(Self)`: otherwise.
///
/// # Panics
/// Does not panic, but `Vec` allocation may panic (until Rust stabilizes fallible collections).
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct NonEmptyRectList2D<T> {
    list: Box<[T]>,
    cols: usize,
    rows: usize,
}

impl<T> NonEmptyRectList2D<T> where T: Copy, {
    /// Constructor
    pub fn new<TIntoNonZeroUsize>(value: T,
                                  rows: TIntoNonZeroUsize,
                                  cols: TIntoNonZeroUsize)
                                  -> Result<Self>
        where TIntoNonZeroUsize: TryInto<NonZeroUsize>,
              <TIntoNonZeroUsize as TryInto<NonZeroUsize>>::Error: Debug,
    {
        let (rows, cols) = Self::into_non_zero_usize(rows, cols)?;
        let elements = rows.checked_mul(cols)
                           .ok_or_else(|| Error::TooManyElements(rows.into(), cols.into()))?;

        (vec![value; elements.into()], rows, cols).try_into()
    }
}

impl<T> NonEmptyRectList2D<T> {
    /// Returns the number of columns (width) of the 2D list.
    #[inline]
    #[must_use]
    pub const fn cols(&self) -> usize { self.cols }

    /// Returns an optional reference to the value at `row`, `col`, if the coordinates exist.
    #[inline]
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        row.checked_mul(self.cols())
           .and_then(|el_row| el_row.checked_add(col).and_then(|idx| self.list.get(idx)))
    }

    /// Sets the value at coordinates `row`, `col` and returns optional `&mut Self` if the
    /// coordinates exist.
    #[inline]
    #[must_use]
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<&mut Self> {
        row.checked_mul(self.cols()).and_then(|el_row| {
                                        el_row.checked_add(col).map(|idx| {
                                                                   self.list
                                                                       .get_mut(idx)
                                                                       .map(|entry| *entry = value);
                                                                   self
                                                               })
                                    })
    }

    /// DRY implementation optionally converting from `TTryIntoNonZeroUsize: TryInto<NonZeroUsize>`
    /// to `NonZeroUsize`, returning either the success value or an error.
    fn into_non_zero_usize<TTryIntoNonZeroUsize>(row: TTryIntoNonZeroUsize,
                                                 col: TTryIntoNonZeroUsize)
                                                 -> Result<(NonZeroUsize, NonZeroUsize)>
        where TTryIntoNonZeroUsize: TryInto<NonZeroUsize>,
              <TTryIntoNonZeroUsize as TryInto<NonZeroUsize>>::Error: Debug,
    {
        let res = (row.try_into().map_err(|err| Error::NoData(format!("{err:?}")))?,
                   col.try_into().map_err(|err| Error::NoData(format!("{err:?}")))?);
        Ok(res)
    }

    /// Returns the number of rows (height) of the 2D list.
    #[inline]
    #[must_use]
    pub const fn rows(&self) -> usize { self.rows }
}

/// Converting Constructor
impl<T, TTryIntoNonZeroUsize> TryFrom<(Vec<T>, TTryIntoNonZeroUsize, TTryIntoNonZeroUsize)>
    for NonEmptyRectList2D<T>
    where TTryIntoNonZeroUsize: TryInto<NonZeroUsize>,
          <TTryIntoNonZeroUsize as TryInto<NonZeroUsize>>::Error: Debug,
{
    type Error = Error;

    fn try_from((vec, rows, cols): (Vec<T>, TTryIntoNonZeroUsize, TTryIntoNonZeroUsize))
                -> std::result::Result<Self, Self::Error> {
        let (rows, cols) = Self::into_non_zero_usize(rows, cols)?;
        let rows = usize::from(rows);
        let cols = usize::from(cols);
        rows.checked_mul(cols)
            .and_then(|elements| (elements == vec.len()).then_some(elements))
            .ok_or(Error::TooManyElements(rows, cols))?;
        let res = Self { list: vec.into_boxed_slice(),
                         rows,
                         cols };
        Ok(res)
    }
}
