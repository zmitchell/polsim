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

The orientation of the beam is completely arbitrary in this situation, so I just chose 45 degrees.
