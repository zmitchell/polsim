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

The key here is that the QWP and the polarizer need to be at 45 degrees relative to on another. If this angle is off, you'll end up with elliptical polarization. Here's what the output looks like:

```
$ polsim circular_polarizer.toml
intensity: 5.00000e-1
x_mag: 5.00000e-1
x_phase: 0.00000e0
y_mag: 5.00000e-1
y_phase: 1.57080e0
```

If you go back through the previous two examples, you'll see that the phases (`x_phase` and `y_phase`) are zero for both examples. This time, however, `y_phase` is not zero, it's `pi/2`! That tells us that there is a delay between the x- and y-components.
