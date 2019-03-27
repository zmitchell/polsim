# polsim
**polsim** is a command line tool for quickly doing polarization simulations with [Jones calculus][jones_calc]. Jones calculus allows you to compute the effect of a sequence of optical elements (polarizers, wave plates, etc) on a beam if the initial state (intensity and polarization) is known. A beam is represented as a vector with two elements (x- and y-components), and an optical element is represented as a 2x2 matrix that operates on the beam/vector. `polsim` makes life easy by letting you skip the tedious work of multiplying the matrices together by hand.

## Elevator Pitch
Would you rather look up this matrix,

\\[
\begin{bmatrix}
\cos^{2}\left(\theta\right) + e^{i\varphi} \sin^{2}\left(\theta\right) & \sin\left(\theta\right)\cos\left(\theta\right) - e^{i\varphi} \sin\left(\theta\right)\cos\left(\theta\right) \\\\
\sin\left(\theta\right)\cos\left(\theta\right) - e^{i\varphi} \sin\left(\theta\right)\cos\left(\theta\right) & \sin^{2}\left(\theta\right) + e^{i\varphi} \cos^{2}\left(\theta\right) \\\\
\end{bmatrix}
\\]

then plug in \\(\theta = \pi/4\\) and \\(\varphi = \pi/2\\), or would you rather write *this*?

```toml
[[elements]]
element_type = "retarder"
phase = 1.57  # pi/2
phase_units = "radians"
angle = 45.0
angle_units = "degrees"
```

## Simulation Library
`polsim` is really just a pretty face for the simulation library I wrote, which can be found [here][polarization]. If you think you've found a bug in the simulation results, you should create an issue on [Github][issue]. The simulation library is tested very thoroughly for correctness, but it's entirely possible that something slipped through the cracks!

If you want to do more complicated simulations (e.g. sweep the angle of a polarizer from 0 degrees to 10 degrees in 0.1 degree increments) you'll have to use the simulation library directly. I hope that you can do that directly from `polsim` one day, but I don't see myself having the time to work on it any time soon.

## Legalese

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE][apache] or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT][mit] or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[jones_calc]: https://en.wikipedia.org/wiki/Jones_calculus
[polarization]: https://github.com/zmitchell/polarization
[issue]: https://github.com/zmitchell/polarization/issues
[mit]: https://github.com/zmitchell/polsim/blob/master/LICENSE-MIT
[apache]: https://github.com/zmitchell/polsim/blob/master/LICENSE-APACHE

