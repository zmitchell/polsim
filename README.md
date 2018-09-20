# PolSim (W.I.P.)

A (W.I.P.) command line utility for doing polarization simulations.

## About

PolSim is a command line utility that wraps a polarization simulation library (found
[here][polarization], also written by me). Here's how it works:

* You specify a beam and some optical elements in a [TOML][toml] file.
* The command line utility, `polsim`, reads the file and performs the simulation.
* The results of the simulation are either printed to the terminal (i.e. `stdout`) or to a file.

For example, the file shown below would specify a linearly polarized beam parallel to
the x-axis, which passes through a quarter-wave plate at a 45 degree angle relative to
the x-axis, and then a linear polarizer oriented parallel to the y-axis.

```toml
# simulation.toml
[[beam]]
polarization = "linear"
angle = 0.0
angle_units = "degrees"

[[elements]]
element_type = "qwp"
angle = 45.0
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 90.0
angle_units = "degrees"
```

You would use `polsim` to read the file and do the simulation like this:

```
$ polsim -i simulation.toml
<simulation results printed here>
```

As mentioned at the beginning, this is a work in progress. I haven't quite decided on
the format of the output yet, but I suspect it will include the magnitudes and phases of
both x- and y-components, as well as the intensity.

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

[polarization]: https://crates.io/crates/polarization
[toml]: https://github.com/toml-lang/toml