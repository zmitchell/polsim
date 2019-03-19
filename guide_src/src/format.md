# File format

An optical system is defined by both a beam and a sequence of optical elements that the beam will pass through. If you run a simulation without defining a beam or without defining your elements, you will get an error explaining what you're missing. Here's the general outline of what the simulation file looks like:

```toml
[beam]
# beam definition goes here

[[elements]]
# the first element goes here

[[elements]]
# the second element goes here

...
```

The simulation file uses a format called TOML, which stands for Tom's Obvious, Minimal Language. I'll explain everything you need to know in this guide, but if you want to read more about TOML, you can do so at the [TOML GitHub repository][toml].

There are two main sections of the simulation file. The first section is the part marked by the `[beam]` heading. The second section is marked by one or more `[[elements]]` headings. The `[beam]` heading marks the definition of the beam that you want to start your simulation with. Each `[[elements]]` heading defines an element in the optical system. The specifics about what is needed to define a beam or an element will be discussed in later sections of this guide.

Just to give you a taste of what a definition looks like, here is the definition of a linearly polarized beam with a polarization vector at 45 degrees measured from the +x-axis:
```toml
[beam]
polarization = "linear"
angle = 45
angle_units = "degrees"
```

The definitions of beams and elements are written out in key-value pairs in the form `key = value`. 

The rule here is that numbers can be written in just about any way you want. For example, `2`, `2.0`, `-2`, and `2.0e-3` are all valid numbers. On the other hand, you can't write mathematical expressions. For example, let's say you want to set a beam at an angle of 45 degrees. It is completely valid to write `angle = 45`, but it is not valid to write `angle = 30 + 15` even though `30 + 15` is obviously equal to `45`. In short, just stick to a single number, and everything should work just fine. There are also values that are not numbers. These values must be enclosed in double quotes e.g. `"linear"`.

Read on to see how to define beams and elements.

[toml]: https://github.com/toml-lang/toml
