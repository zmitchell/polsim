# Crossed Polarizers

If I orient two polarizers perpendicular to one another, no beam will be able to pass through. Here's what that looks like:

```toml
[beam]
polarization = "linear"
angle = 45
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 0
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 90
angle_units = "degrees"
```

The orientation of the beam is completely arbitrary in this situation, so I just chose 45 degrees. Here's what the output looks like:

```
$ polsim crossed_polarizers.toml
intensity: 1.87470e-33
x_mag: 2.65123e-33
x_phase: 0.00000e0
y_mag: 4.32978e-17
y_phase: 0.00000e0
```

Since computers can't represent numbers with infinite precision, you're likely to see something weird like this if your numbers are close to zero. We're all physicists here, so we know that anything as small as `1e-17` or `1e-33` is basically zero anyway when compared to 1 (the intensity of the original beam).

We can use Malus's Law again (see the previous example) to verify that this is correct. The beam will have polarization at 0 degrees after it passes through the first polarizer. The second polarizer is at 90 degrees, meaning that the angle between the beam and the second polarizer is also 90 degrees. The cosine of 90 degrees is zero, so no light (ideally) passes through the second polarizer.
