# Installation

The first step is to head to the [GitHub repository][releases] and download the latest release for your operating system. `polsim` is only available for 64-bit operating systems because it's 2019. Here is what the filename will look like for each supported operating system:
- Windows: `polsim-<version>-x86_64-pc-windows-msvc.zip`
- macOS: `polsim-<version>-x86_64-apple-darwin.tar.gz`
- Linux: `polsim-<version>-x86_64-unknown-linux-gnu.tar.gz`

The next step is to extract the archive you just downloaded and put the executable file somewhere. For Windows, the executable will be called `polsim.exe`, and for macOS and Linux the executable will be called `polsim`. `polsim` doesn't need to be put anywhere special, so you can put it wherever you want.

`polsim` is a command line program, so you'll be using a shell (`cmd.exe` on Windows, Terminal on macOS, etc) to interact with it. Your shell needs to know where to find `polsim` in order to use it, so you have two options:

- Put `polsim` in a directory, then avigate to this directory each time you want to use `polsim`.
- Put `polsim` in a directory, then add that directory to your shell's `PATH` so you can use `polsim` from any directory.

The second option is more convenient (in my opinion), so it's the recommended option. If you need help adding a directory to your shell's `PATH`, here are some guides for each supported operating system:

- [Windows][windows]
- [macOS][macos]
- [Linux][linux]

[releases]: https://github.com/zmitchell/polsim/releases
[windows]: https://helpdeskgeek.com/windows-10/add-windows-path-environment-variable/
[macos]: http://osxdaily.com/2014/08/14/add-new-path-to-path-command-line/
[linux]: https://www.techrepublic.com/article/how-to-add-directories-to-your-path-in-linux/
