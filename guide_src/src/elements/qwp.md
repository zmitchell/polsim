# Quarter-Wave Plate

If you fix the phase delay introduced by a retarder to `pi/2`, or one quarter of a wavelength, you get a quarter-wave plate. This kind of element is used to convert between linear and circular polarization.

The definition of a quarter-wave plate only needs an angle since the phase is a fixed value.

```toml
[[elements]]
element_type = "qwp"
angle = 45
angle_units = "degrees"
```
