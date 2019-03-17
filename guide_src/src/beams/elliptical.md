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
