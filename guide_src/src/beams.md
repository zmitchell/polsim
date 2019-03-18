# Beams

The section for defining a beam is marked by `[beam]`. Notice that there's only **one** set of braces around `[beam]` as opposed to **two** set of braces around `[[elements]]`. In TOML we say that `[beam]` is a table, and `[[elements]]` is an array (a list of values, in this case a list of elements).

The `[beam]` table can take a few different key-value pairs depending on how you want to define your beam. When you really get down to it, you're just defining the x- and y-components of the polarization, but that's overly tedious for most cases. To make life easier for you, I've provided you with some shortcuts for linearly and circularly polarized beams. You can, however, still define the beam in terms of the x- and y-components of the polarization if you really want to.

One important thing to mention before we move on is that no matter how you define your beam, it's initial intensity will always be 1.

The next few sections will describe how to define beams with different types of polarization.
