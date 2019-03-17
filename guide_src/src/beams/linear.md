# Linearly Polarized

Let's start out by defining a linearly polarized beam with horizontal polarization. Here is the whole definition:

```toml
[beam]
polarization = "linear"
angle = 0
angle_units = "degrees"
```

That's not so bad, is it?

The first key is `polarization`. Every beam definition needs a `polarization` key because it tells the simulation what other keys to expect in the beam definition. For example, it doesn't make sense to specify the angle of a circularly polarized beam, so you aren't asked for `angle` or `angle_units` when `polarization = "circular"`. We're getting ahead of ourselves though, so let's get back to our linearly polarized beam.

The definition of a linearly polarized beam just needs an angle to describe the orientation of the polarization vector. An angle is defined with two keys: `angle` and `angle_units`. The `angle` key takes the actual value of the angle, and the `angle_units` key specifies the units in which `angle` is given. The `angle_units` key can take the following values:
- `"degrees"`
- `"radians"`

Every angle definition requires you to specify the units. This might seem tedious, but there are good reasons for this choice. Sometimes it's more convenient or more natural to express an angle in degrees or in radians, so you are free to mix and match which units you use for the various angles that you specify in your simulation file. However, since you have the freedom to mix and match the units, you also have the freedom to mix them up by mistake. Making you be explicit about the units prevents mistakes. This is science after all, it's supposed to be correct!
