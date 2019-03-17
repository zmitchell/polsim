# Elements
Recall that you specify the elements of the optical system with one or more sections that start with `[[elements]]`. In TOML we say that each `[[foo]]` defines an item in an array (list) called `foo`. In our case we're defining the items (optical elements) in an array called `elements`. The order in which you define the elements is the order in which the beam will travel through the elements. This makes a big difference in your simulation!

Much like the definition of a beam, the definition of an element is written out using key-value pairs. Different elements will require different key-value pairs in their definition, but they will all have an `element_type` key. The list of possible element types is as follows:
- polarizer: `"polarizer"`
- polarization rotator: `"rotator"`
- retarder: `"retarder"`
- quarter-wave plate: `"qwp"`
- half-wave plate: `"hwp"`

Several elements require an angle as part of their definition, but an angle can mean different things in the context of different optical elements. I'll point out the differences for each element, but it's importat to keep track of what each angle means. The way that you define an angle for an element is exactly the same way that you define an angle for a beam (e.g. with `angle` and `angle_units`). The same goes for `phase` and `phase_units`.

Let's look at how you define each element type.
