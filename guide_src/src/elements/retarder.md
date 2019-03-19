# Retarder
A retarder is an optical element that introduces a phase difference between two perpendicular components of a beam's polarization. We say that the phase of one of the components has been retarded relative to the other, hence the name "retarder". The polarization component perpendicular to the "fast" axis of the retarder will lag behind the other component by some fixed phase specific to the particular retarder.

A retarder is more complicated than the other elements we've discussed here, so I'll direct you to the wonderful [RP Photonics Encyclopedia][rp-photonics] to learn more about how a retarder works under the hood.

The effect that the retarder has on the beam depends on two things:
1. The orientation of the beam's polarization relative to the orientation of the retarder's "fast" axis.
2. The phase delay introduced by the retarder.

In your retarder definition you'll have to specify both the orientation of the element and the phase delay introduced by the element. A phase is really just an angle, so the definition of a phase works exactly the same way as a definition of an angle. They even use the same units!

```toml
[[elements]]
element_type = "retarder"
angle = 45
angle_units = "degrees"
phase = 1.5705
phase_units = "radians"
```

[rp-photonics]: https://www.rp-photonics.com/waveplates.html
