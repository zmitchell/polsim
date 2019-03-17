# Circular Polarizer

If you have a linearly polarized beam you can convert it into a circularly polarized beam with a polarizer and a QWP. Here's what that looks like:

```toml
[beam]
polarization = "linear"
angle = 90
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 45
angle_units = "degrees"

[[elements]]
element_type = "qwp"
angle = 0
angle_units = "degrees"
```

The key here is that the QWP and the polarizer need to be at 45 degrees relative to on another. If this angle is off, you'll end up with elliptical polarization.
