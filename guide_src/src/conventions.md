# Conventions

## Definitions
### Polarization Orientation
Throughout this documentation you might see me say that the orientation of a beam is 45 degrees. There's a number of different ways to interpret this, so let's spell out exactly what I mean.

- The beam is traveling away from me.
- The +x-axis is pointed to the right, and corresponds to an angle of zero.
- All other angles are measured counter-clockwise from the +x-axis.

For example, if I say that a beam has an orientation of 90 degrees, that means that it is oriented along the +y-axis. In actuallity, the polarization of a beam is a line that extends out to infinity in both directions, so the polarization in the previous example would extend along both the +y- and -y-axes.

### Handedness
The handedness of a circularly polarized beam is defined using the right-hand rule. If the beam is traveling away from you, the polarization vector of a "right"-hand circularly polarized beam will rotate clockwise in the plane of the polarization.

## Reports
## Relative Phases
The output of `polsim` will follow the conventions of Jones calculus with regards to relative phases. This means that the phase of the x-component will be factored out from both the x- and y-components, leaving the x-component as a real number, and potentially leaving the y-component with some phase relative to the x-component.

## Intensities
All beams start with an intensity of 1. If the output of your simulation shows `intensity: 5.00000e-1`, that means that the intensity of the beam at the end of the simulation is half of the original intensity.

## Phases
The phases that you see at the end of a simulation are always in radians.
