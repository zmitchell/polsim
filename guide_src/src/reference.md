# Quick Reference

### File Format
```toml
[beam]
# beam definition goes here

[[elements]]
# the first element goes here

[[elements]]
# the second element goes here

...
```

## Angles
`angle` may be an integer or a floating point value.

`angle_units` may take the following values:
- `"degrees"`
- `"radians"`

## Handedness
`handedness` may take the following values:
- `"left"`
- `"right"`

## Phase
Phases are really just angles, so they follow exactly the same rules as angles.

`phase` may be an integer or a floating point value.

`phase_units` may take the following values:
- `"degrees"`
- `"radians"`

## Polarization
`polarization` may take the following values:
- `"linear"`
- `"circular"`
- `"elliptical"`

## Element Types
`element_type` may take the following values:
- `"polarizer"`
- `"retarder"`
- `"rotator"`
- `"qwp"`
- `"hwp"`

## Beams
### Linearly Polarized
Linear polarization requires an angle.

```toml
[beam]
polarization = "linear"
angle = 0.0
angle_units = "degrees"
```

### Circularly Polarized
Circular polarization requires a handedness.

```toml
[beam]
polarization = "circular"
handedness = "left"
```

### Elliptically Polarized
Elliptical polarization should be used when you want to define an arbitrary beam. This definition requires the magnitude and phase of the x- and y-components, and the units used in the phase definitions.

```toml
[beam]
polarization = "elliptical"
x_mag = 1.0
x_phase = 0.0
y_mag = 1.0
y_phase = 3.141
phase_units = "radians"
```

## Elements
