# Polarization Rotator

A polarization rotator is pretty much what it says on the tin: it rotates the polarization of a beam.
 
The definition of a polarization rotator needs an angle, but in this case the angle isn't the orientation of the element. The angle specified here is the angle by which the polarization will be rotated. The convention is that positive angles correspond to rotating the polarization counter-clockwise.

Here's an example of an element that rotates the polarization of a beam by 90 degrees:
```toml
[[elements]]
element_type = "rotator"
angle = 90.0
angle_units = "degrees"
```
