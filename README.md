# polsim

[![Build Status](https://travis-ci.com/zmitchell/polsim.svg?branch=master)](https://travis-ci.com/zmitchell/polsim)

A command line utility for doing polarization simulations with [Jones calculus][jones_calc].

For installation and usage instructions, see the [User Guide][guide].

## About

`polsim` is a command line utility that wraps a polarization simulation library (found
[here][polarization], also written by me). Both the command line utility and the polarization simulation library are written in [Rust][rust]. Here's how it works:

* You specify a beam and some optical elements in a [TOML][toml] file.
* The command line utility reads the file and performs the simulation.
* The results of the simulation are printed to the terminal.

For example, to specify a linearly polarized beam oriented at 0 degrees (i.e. horizontally) that passes through a polarizer oriented at 45 degrees, your file would look like this:

```toml
# simulation.toml
[beam]
polarization = "linear"
angle = 0.0
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 45.0
angle_units = "degrees"
```

You would use `polsim` to read the file and do the simulation like this:

```
$ polsim simulation.toml
intensity: 5.00000e-1
x_mag: 5.00000e-1
x_phase: 0.00000e0
y_mag: 5.00000e-1
y_phase: 0.00000e0
```

If you want to impress your friends and family, you can print the results in a table via the `-p/--pretty` flag:
```
$ polsim -p simulation.toml
+------------+------------+-----------+------------+-----------+
| intensity  | x_mag      | x_phase   | y_mag      | y_phase   |
+------------+------------+-----------+------------+-----------+
| 5.00000e-1 | 5.00000e-1 | 0.00000e0 | 5.00000e-1 | 0.00000e0 |
+------------+------------+-----------+------------+-----------+
```

For more information, see the [User Guide][guide].

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[jones_calc]: https://en.wikipedia.org/wiki/Jones_calculus
[polarization]: https://crates.io/crates/polarization
[toml]: https://github.com/toml-lang/toml
[rust]: https://www.rust-lang.org/
[guide]: www.example.com
