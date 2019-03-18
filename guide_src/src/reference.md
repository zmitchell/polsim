# Quick Reference

> Note: In the definitions that follow, text in `<angle brackets>` are placeholders for values that you must supply.

### File Format
```toml
[beam]
# beam definition goes here
polarization = <polarization>
<polarization-specific keys>

[[elements]]
<element definition>

[[elements]]
<element definition>

...
```

## Angles
- `angle`
    - Takes an integer or floating point number.
- `angle_units`
    - `"degrees"`
    - `"radians"`

Example:
```toml
angle = 0
angle_units = "degrees"
```

## Phase
Phases are really just angles, so they follow exactly the same rules as angles.

- `phase`
    - Takes an integer or floating point number.
- `phase_units`
    - `"degrees"`
    - `"radians"`

Example:
```toml
phase = 3.141
phase_units = "radians"
```

## Handedness
- `handedness`
    - `"left"`
    - `"right"`

Example:
```toml
[beam]
polarization = "circular"
handedness = "left"
```

## Polarization
- `polarization`
    - `"linear"`
    - `"circular"`
    - `"elliptical"`

Example:
```toml
[beam]
polarization = "circular"
handedness = "left"
```

## Beams
### Linearly Polarized
```toml
[beam]
polarization = "linear"
angle = <number>
angle_units = <angle units>
```

Example:
```toml
[beam]
polarization = "linear"
angle = 0
angle_units = "degrees"
```

### Circularly Polarized
```toml
[beam]
polarization = "circular"
handedness = <handedness>
```

Example:
```toml
[beam]
polarization = "circular"
handedness = "left"
```

### Elliptically Polarized
```toml
[beam]
polarization = "elliptical"
x_mag = <number>
x_phase = <number>
y_mag = <number>
y_phase = <number>
phase_units = <angle units>
```

Example:
```toml
[beam]
polarization = "elliptical"
x_mag = 1
x_phase = 0
y_mag = 1
y_phase = 3.141
phase_units = "radians"
```

## Element Types
- `element_type`
    - `"polarizer"`
    - `"retarder"`
    - `"rotator"`
    - `"qwp"`
    - `"hwp"`

Example:
```toml
[[elements]]
element_type = "polarizer"
angle = 0
angle_units = "degrees"
```

## Elements

### Polarizer
```toml
[[elements]]
element_type = "polarizer"
angle = <number>
angle_units = <angle units>
```

Example:
```toml
[[elements]]
element_type = "polarizer"
angle = 0
angle_units = "degrees"
```

### Polarization Rotator
```toml
[[elements]]
element_type = "rotator"
angle = <number>
angle_units = <angle units>
```

Example:
```toml
[[elements]]
element_type = "rotator"
angle = 45
angle_units = "degrees"
```

### Retarder
```toml
[[elements]]
element_type = "retarder"
angle = <number>
angle_units = <angle units>
phase = <number>
phase_units = <phase units>
```

Example:
```toml
[[elements]]
element_type = "retarder"
angle = 45
angle_units = "degrees"
phase = 3.141
phase_units = "radians"
```

### Quarter-Wave Plate
```toml
[[elements]]
element_type = "qwp"
angle = <number>
angle_units = <angle units>
```

Example:
```toml
[[elements]]
element_type = "qwp"
angle = 45
angle_units = "degrees"
```

### Half-Wave Plate
```toml
[[elements]]
element_type = "hwp"
angle = <number>
angle_units = <angle units>
```

Example:
```toml
[[elements]]
element_type = "hwp"
angle = 45
angle_units = "degrees"
```
