# One Polarizer

Let's say I have a linearly polarized beam with vertical polarization. I want to pass that polarization through a polarizer oriented at 45 degrees. Here's what that looks like:

```toml
[beam]
polarization = "linear"
angle = 90
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 45
angle_units = "degrees"
```

