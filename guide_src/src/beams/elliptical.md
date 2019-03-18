# Elliptically Polarized

If you want to define an arbitrary beam, you can do so with the `"elliptical"` polarization type. For this beam definition there are no shortcuts, you have to specify everything yourself.

```toml
[beam]
polarization = "elliptical"
x_mag = 1.0
y_mag = 1.0
x_phase = 0.0
y_phase = 3.141
phase_units = "radians"
```

The `x_mag` and `y_mag` keys are the magnitudes of the x- and y-components of the polarization. The `x_phase`, `y_phase`, and `phase_units` keys specify the phases of the components. In `polsim` phases are just like angles. The phases are even specified with the same units as angles.

This is the one exception to the "specify the units of every angle" rule. Both `x_phase` and `y_phase` must have the same units. I don't know why you would define `x_phase` with one set of units and `y_phase` with another set of units, so I'm just not going to let you do that.

The last thing to mention is that the intensity of the beam will be 1 no matter what magnitudes you provide in `x_mag` and `y_mag`. This was mentioned earlier, but it's worth repeating to avoid any potential confusion. You can still provide magnitudes that are larger than 1 (I even did that in the example above), and it can even be convenient to do so, just don't be surprised if your intensity is never larger than 1!

If you supply magnitudes larger than 1, they will just be normalized so that the intensity is 1. Let's walk through what happens in the example given above. First, the intensity is calculated from the provided magnitudes:

```
x_mag^2 + y_mag^2 = initial_intensity
1 + 1 = 2
initial_intensity = 2
```

Then each magnitude is divided by the square root of this initial intensity. This is more or less the same thing as dividing both sides of the above equation by `initial_intensity`.

```
normed_x_mag = x_mag / sqrt(initial_intensity)
normed_y_mag = y_mag / sqrt(initial_intensity)
normed_x_mag = 1 / sqrt(2) = 0.707
normed_y_mag = 1 / sqrt(2) = 0.707
```

Now the intensity will be 1 when computed from the normalized components:

```
normed_x_mag^2 + normed_y_mag^2 = intensity
(0.707^2) + (0.707^2) = intensity
0.5 + 0.5 = intensity
1 = intensity
```
