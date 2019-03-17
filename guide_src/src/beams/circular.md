# Circularly Polarized

The definition of a circularly polarized beam is even simpler than that of a linearly polarized beam:
```toml
[beam]
polarization = "circular"
handedness = "right"
```

A circularly polarized beam is defined by its handedness i.e. which direction the polarization vector rotates as the beam propagates (clockwise or counter-clockwise). The convention for the "handedness" of a circularly polarized beam is defined by the "right-hand rule", that is, a beam is "right" handed if the polarization vector appears to rotate clockwise as the beam travels away from you. 

The `handedness` key can take the following values:
- `"left"`
- `"right"`

If you know someone who has a hand that isn't a right or left hand, please let me know.
