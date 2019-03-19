# Polarizer

A polarizer is an optical element that only lets polarization of a certian orientation pass through. Think about it like this: 

> You have a wall full of toasters and a bunch of bread. You want to make some toast, so you throw some bread at your toaster-wall. The only slices of bread that make it into a toaster are the slices of bread that are lined up with the slots on the top of a toaster.

It's the same thing with light and a polarizer. Exactly the same.

The definition of a polarizer just needs an angle. The angle in this case is the orientation of the polarization that the polarizer will let through. Here's an example:

```toml
[[elements]]
element_type = "polarizer"
angle = 45
angle_units = "degrees"
```

This definition corresponds to an ideal linear polarizer that makes a 45 degree angle with the +x-axis.
 
