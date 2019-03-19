# Usage

> Note: This section assumes that you've already installed `polsim`. If you haven't installed `polsim`, you can find instructions in the [Installation][installation] section.

> Note: Some of what follows will be things that you type into a shell/terminal. This will be `cmd.exe` on Windows, `Terminal.app` on macOS, or your terminal of choice on Linux. The parts that you type into a terminal will appear like this:
>
>```
>$ some commands to execute
>```
>
>In these parts the `$` just indicates the prompt in your shell, so you don't need to type the `$` character. Some instructions will contain text in `<angle brackets>`. The text in angle brackets will be a placeholder for something that you need to supply e.g. a filename or a path to a directory.



## Your first simulation

Let's get things started with an example. Put the following text into a file called `simulation.toml` and save it somewhere.

```toml
# simulation.toml
[beam]
polarization = "linear"
angle = 90
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 45
angle_units = "degrees"
```

Next, open up your shell and navigate to the directory in which you saved `simulation.toml`.

```

$ cd <path to the directory>
```

Now we're going to tell `polsim` to run a simulation with this file.

```
$ polsim simulation.toml
```

When you run it you should see the following output:

```
$ polsim simulation.toml
intensity: 5.00000e-1
x_mag: 5.00000e-1
x_phase: 0.00000e0
y_mag: 5.00000e-1
y_phase: 0.00000e0
```

You've just run your first simulation! Let's dig in and see what this output is telling us.

## Interpreting the results

What you see at the end of a simulation is a summary of the beam after it has passed through all of the elements that you defined in your optical system. The report includes the intensity of the beam as well as the magnitude and phase of both components of the complex polarization vector.

```
intensity: 5.00000e-1
```

All beams in `polsim` start with an intensity of 1. We see here that the intensity is 0.5, which tells us that the beam has half the intensity that it started out with.

```
x_mag: 5.00000e-1
x_phase: 0.00000e0
```

The components of an electric field are, in general, complex. It's typically easier to reason about the components of the electric field in terms of magnitude and phase rather than the real and imaginary parts. What you see here as `x_mag` and `x_phase` can be written mathematically as:

\\(E_x = r e^{i\varphi} = x\\_mag\\,e^{i\\,x\\_phase}\\)

The same idea applies to `y_mag` and `y_phase`, obviouslly. 

Another thing to notice is that the x- and y-components are not normalized to 1, they are normalized such that `x_mag^2 + y_mag^2 = intensity`.

## Troubleshooting

We haven't discussed how to write your own simulations yet, so don't worry too much about understanding exactly what the simulation file says. For now, just try to follow along and get a sense for how `polsim` will try to help you out when you make mistakes. Don't worry though, we'll talk about the nitty gritty details of how to make your own simulations soon enough.

Let's see what happens when you make an error in your simulation file. We're going to delete the last line of `simulation.toml` and see what `polsim` has to say about that. Copy and paste the following text into a file called `has_error.toml`, then save it.

```toml
# has_error.toml
[beam]
polarization = "linear"
angle = 90
angle_units = "degrees"

[[elements]]
element_type = "polarizer"
angle = 45
```

Now we're going to (try to) run a simulation with this file.

```
$ polsim has_error.toml
error: invalid system definition
caused by: invalid element definition
caused by: invalid polarizer definition
caused by: invalid angle definition
caused by: missing parameter in definition: 'angle_units'
```

That's a lot of output, but if you read it from the top down it will help you pinpoint where the error came from. Let's break it down line by line.

```
error: invalid system definition
```

This says that there was an error in the definition of our simulation. In other words, there's something wrong with what we typed into `has_error.toml`, but we don't quite know what just yet.

```
caused by: invalid element definition
```

Now we know that there's something wrong with one of the elements that we defined. We still don't know which one is the source of the problem though.

```
caused by: invalid polarizer definition
```

Aha, now we're getting somewhere. This line tells us that there's an issue with a polarizer. Our simulation only has one polarizer, so that's enough to tell us which element is the problem. If your simulation has more than one of the same type of element, you'll have to do a bit more sleuthing to figure out which one it is. What's wrong with the definition of our polarizer?

```
caused by: invalid angle definition
```

Something seems to be wrong with the definition of the angle of the polarizer. Let's see what's up.

```
caused by: missing parameter in definition: 'angle_units'
```

This is the source of the problem. Every angle that you define needs two pieces: `angle` and `angle_units`. Our problematic simulation file, `has_error.toml`, left out the `angle_units` for the definition of our polarizer.

Most error messages that you receive should be helpful in pinpointing your error. If you get an error message that sounds like gibberish, feel free to post an issue on the [GitHub repository][issues].

[installation]: installation.md
[issues]: https://github.com/zmitchell/polsim/issues
