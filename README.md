# islands
An implementation of the Islands problem as presented by Jim Bennett & Ryan Levick on 
[YouTube](https://www.youtube.com/watch?v=ugz1YgoZmzI).

The premise:
> Given a grid of locations which can either be land or water, determine how many islands are on the grid.
 
I wanted to write it as an industrial-strength solution to aid in conversations and
mentoring work that I do with other Rustaceans.

It is designed to be/have:
  * **Panic-free**: Out-of-memory conditions can still fail the algorithm (it inserts into a `HashSet`).
      This situation will persist until Rust stabilizes fallible collections.

      NOTE: This implementation is recursive to better relate to the YouTube stream, and so will run 
      out of (configurable) stack space if the `World` map is large enough (~1300 elements in either
      dimension with the default stack).  It is possible to translate this implementation to a loop
      to eliminate this concern.


  * **Declarative/functional**: By focusing on shaping the data, instead of
      "micromanaging CPU state", the program is subject to far fewer bugs, is more expressive and
      tends to compile to tighter code in release mode.  Chief drawbacks to this include i) lower 
      familiarity in the general engineering populace/deeper software engineering experience 
      required to read and write this way and ii) debug tools remain optimized for imperative-style
      code, which can make intermediate fluent values hard to access. 


  * **Strong type-safety**: By encoding invariants (things that need to be true in order for the program 
      to work correctly) into data types using the type system, users of these types will receive
      more compile-time errors, which prevents bugs.  It also lowers the burden of development,
      since empty lists, rectangular lists, dimension overflow and other issues are all handled at 
      `World`'s construction time.  If it exists, it is valid.  This frees developers of all other
      code using `World`, (in this small example) from the burden of having to remember when to 
      check all of the various invariants, each time the code is touched, forever.
  

  * **Efficient**: Representing the world internally as a 1D contiguous `Box`ed  `slice` of values gives
      good default CPU cache-efficiency.  Compare this with a `Vec<Vec<T>>`, where each inner
      `Vec<T>` is a distinct heap allocation with no promise of locality.  A `[[T; COLS]; ROWS]` is
      contiguous and efficient, but requires `const` (compile-time) dimensions, which may be 
      limiting for some use cases.


  * **Ergonomic**: Users of the `count_islands()` function simply pass in a map.  Internal tracking
      of visited locations is handled... internally.  The `World` type, although internally
      implemented as a 1D array, presents a 2D interface in the `is_land()` method, making it
      "pit-of-success" convenient to use without leaking the 1D `slice` abstraction.

      This implementation does not attempt to provide `World` converting constructors from 
      "convenient-but-performance-eating-antipattern" types like `Vec<Vec<T>>`.  This is a form of
      opinionated API which is a weak pit-of-success forcing function.

  * **Compiler warnings turned up to 11**: Putting the compiler in strict mode surfaces more
      warnings.  If one accepts that "warnings are just errors that haven't happened yet", dealing
      with them from the beginning yields more resilient code and in my experience, the additional
      constraints can drive the discovery of more expressive designs.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
