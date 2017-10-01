# i3lock-slick
A CLI tool to easily make i3lock look [slick](http://www.urbandictionary.com/define.php?term=slick).

The concept if [i3lock](https://i3wm.org/i3lock/) is awesome, but it looks quite ugly by itself.  

Projects like [i3lock-color](https://github.com/chrjguill/i3lock-color) allow visual customizations,
but using these in a nice way is hard and requires additional scripts to be written.

This tool makes things like a blurred desktop as background in `i3lock` very easy,
along with many other styles.

Invoking `i3lock-slick` parses all given arguments and immediately spawns the lock screen accordingly with the proper configuration,
unless a parameters changes this behaviour.
This makes invoking and styling `i3lock` with a single command a piece of cake.

## Usage
```
# Help and version information
i3lock-slick --help
i3lock-slick --version

# Show the lock screen, use the defaults
i3lock-slick

# Show the lock screen, process i3lock parameters
i3lock-slick -p color=ff0000 -p ignore-empty-password
```

## Requirements
Usage requirements:
* `i3lock`: i3lock itself

Development requirements:
* Rust 1.19

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
