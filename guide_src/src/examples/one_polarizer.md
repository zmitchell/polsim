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

Here's what the output looks like:
```
$ polsim one_polarizer.toml
intensity: 5.00000e-1
x_mag: 5.00000e-1
x_phase: 0.00000e0
y_mag: 5.00000e-1
y_phase: 0.00000e0
```

You can see that the intensity is 0.5, which is half of the original intensity since all beams start with an intensity of 1. We can verify that this is correct using [Malus's Law][malus]. Malus's Law says that a beam's intensity after it passes through a polarizer depends on the angle between them:

\\( I = I_0 \cos^{2}\left(\theta\right)\\)

Since the angle between the polarizer and the beam's polarization is 45 degrees, \\( I = 0.5 I_0 \\).

[malus]: https://en.wikipedia.org/wiki/Polarizer#Malus's_law_and_other_properties
