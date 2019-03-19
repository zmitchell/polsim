# polsim

[![Build Status](https://travis-ci.com/zmitchell/polsim.svg?branch=master)](https://travis-ci.com/zmitchell/polsim)

For installation and usage instructions, see the [User Guide][guide].

## Elevator Pitch

Would you rather look up (or calculate) the Jones matrix for an optical retarder oriented at an arbitrary angle with an arbitrary phase delay, or would you rather just type this:

```toml
[[elements]]
element_type = "retarder"
phase = 1.57  # pi/2
phase_units = "radians"
angle = 45.0
angle_units = "degrees"
```

## About

`polsim` is a command line tool for quickly doing polarization simulations with [Jones calculus][jones_calc]. Jones calculus allows you to compute the effect of a sequence of optical elements (polarizers, wave plates, etc) on a beam if the initial state (intensity and polarization) is known. A beam is represented as a vector with two elements (x- and y-components), and an optical element is represented as a 2x2 matrix that operates on the beam/vector. `polsim` makes life easy by letting you skip the tedious work of looking up and multiplying the matrices together by hand.

Here's how it works:

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
[guide]: https://zmitchell.github.io/polsim
