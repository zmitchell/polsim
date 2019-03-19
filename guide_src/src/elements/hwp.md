# Half-Wave Plate

A half-wave plate (HWP) is just like a QWP with a different. This type of element is often used to rotate the polarization of a beam.

Just like a quarter-wave plate, you only need to specify an angle in your definition since the phase is a fixed value.

```toml
[[elements]]
element_type = "hwp"
angle = 45.0
angle_units = "degrees"
```
