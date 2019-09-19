# Brew calculator

Brew calculator backend, implemented in Rust.
The intention is to collect all beer brewing calculations and formulas in one library.

Look, the actual reason is that we at Bryggans Bryggeri needed a native Rust implementation
of some simple brew calculations for our brewery software backend
[bryggio](https://github.com/bryggansbryggeri/bryggio).

But then we got thinking about the rather opaque alternatives that do exist,
either in closed source brewing software or in random browser calculators.

We realised it would actually be neat to spend some time on this library and develop it
guided by the following principles

(**Note:** These are design *goals* and are as such not guaranteed to be implemented yet.
See [current status](#current-status)
):

## Open and common
The library will be free for all to use and modify as long as they display their use of it.
This will ensure a set of accessible, standardised and transparent calculations.

The external API ought to be easy to stabilise,
since it really is a predefined set of formulas and calculations that are exposed.
Then bug fixes and other updates can be pushed without breaking changes.

## Opinionated
How standardise, when there is no standard?
Regarding the unit system, this library will have an opinion.
It will default to
[SI units](https://www.lne.fr/en/learn-more/international-system-units/introduction-si),
if there isn't a very compelling argument against it.
E.g. it seems overly zealous to use degrees Kelvin instead of Celsius and the latter should be favoured.
Conversions to Fahrenheit though, will of course not be implemented.

## Well-documented and tested
Now, how do you calculate the IBU?
It is surprisingly tricky to find a conclusive answer on internet.com.
We will strive to have excellent documentation so that the library can also function as a reference.
Useful: https://docs.rs/rustdoc-katex-demo/0.1.5/rustdoc_katex_demo/

Extensive testing is easy since it really is only a set of formulas,
for which an infinite amount of test cases are readily available.

Using Rust we can also accomplish formal testing:

### Dimensionality safety
Leveraging Rust's type safety the aim is to ensure that the calculations can be trusted.
All quantities, even though they can be represented as ordinary numbers,
have their own type and unit of measurement.
This gives the ability to catch dimensionality errors at compile time.

For instance, adding batch volume to malt weight will simply not compile.

This property will be immensly helpful when doing Rust development with
this library as a dependency.

It will not explicitly affect API users in other languages,
but it will serve as a very strict guarantee that the formulas are proven
to be free from a big class of errors.

The crate
[dimensioned](https://docs.rs/dimensioned/0.7.0/dimensioned/)
seems to do a lot of what we want.
We need to weigh the benefits of dim. safety against crate bloat and ergonomy.

### Purely functional
It would have great benefits to implement this as a purely functional library.
Given the simple, functional nature of the problem, this should be trivial.
There might appear cases where it conflicts with the (external) ergonomy,
but it should be a design principle.

## Ergonomic

While safety and correctness are the primary design principles,
an ergonomic API is important for usability.
As much of the complexity involved in the dimensionality safety should be hidden wherever possible.

## Rust library
The calculations will be distributed in a Rust library on crates.io.

Possibly also as a binary shared lib for some common architectures (Raspberry pi et.c.)

## Javascript through WebAssembly
Dear as it is to us developers, Rust is still somewhat of a fringe language.
Especially given that most existing brew calculators come in the form of web forms.
For maximum reach the goal is to always ensure that the library compiles to webassembly,
in combination with an ergonomic JS API for ease of use.

The resulting library might or might not be more efficient than a native JS implementation but that is not the motivation for webassembly.
Possible performance diffenerences for these very simple calculations will be neglible anyway.
It is rather the benefit of having the same backend being used, regardless if it's for a frontend app or a brewery program backend.

# Current status

- [ ] Implements all necessary functions with simple floats
- [ ] Documented
- [ ] Tested
- [ ] Type safety
- [ ] Dimensionality safe
- [x] Rust library
- [ ] Purely functional
- [ ] Compiles to JS/Wasm
