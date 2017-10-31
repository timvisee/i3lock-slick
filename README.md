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

# Show the lock screen, apply some filters
i3lock-slick --filter blur:sigma=3 --filter greyscale

# Show the lock screen, process i3lock parameters
i3lock-slick -p color=ff0000 -p ignore-empty-password
```

## Installation
The tool is currently installed using Rust's `cargo`.

```
git clone https://github.com/timvisee/i3lock-slick.git
cd i3lock-slick
cargo install
```

Packages will be created at a later time.

## Filters
The following filters are currently available:

- `blur`: blur the image
    - `sigma=float`: (default: 5) blurring intensity as a `float`.
- `brighten`: brighten the image by the given amount.
    - `amount=int`: brightening amount as an `int`, negative for darkening.
- `contrast`: change the image contrast.
    - `amount=int`: adjust the contrast of this image.
        Use `amount` to adjust the contrast by.
        Negative values decrease the contrast and positive values increase the contrast.
- `crop`: cut out the image by the bounding rectangle.
    - `x=int`: top-left x coordinate of the rectangle in pixels.
    - `y=int`: top-left y coordinate of the rectangle in pixels.
    - `width=int`: width of the rectangle in pixels.
    - `height=int`: height of the rectangle in pixels.
- `fliph`: flip the image horizontally.
- `flipv`: flip the image vertically.
- `greyscale`: greyscale the image.
- `huerotate`: hue rotate the image.
    - `amount=int`: hue rotate the supplied image.
        The `amount` is how much degrees to rotate each pixel by.
        0 and 360 do nothing, the rest rotates by the given degree value.
- `invert`: invert the image colors.
- `resize`: resize the image.
    - `width=uint`: the new width in pixels.
    - `height=uint`: the new height in pixels.
    - `ratio=bool`: `false` (default) resize to the exact new dimensions.
        `true` to keep the aspect ratio, which makes the image fit in the given dimensions.
    - `filter=string`: image filter to use for resizing.
        May be one of:
        - `nearest` | `near`: nearest neighbour filter.
        - `trinagle`: (default) linear filter.
        - `catmullrom`: cubic filter.
        - `gaussian`: gaussian filter.
        - `lanczos3`: lanczos filter with window 3.
- `unsharpen`: performs an unsharpen mask on this image.
    - `sigma=float`: the amount to blur the image by.
    - `threshold=int`: control of how much to sharpen.

## Dotfiles
The tool has support for dotfiles,
which may fully configure i3lock-slick as required and eliminates the need of
using command line arguments.

A dotfile template can be found here: [.i3lock-slick.yml](.i3lock-slick.yml)

The following dotfile paths are scanned in order:
- `~/.config/i3lock-slick/config.yml`
- `~/.i3lock-slick.yml`

## Requirements
* `i3lock`: i3lock itself
* `scrot`: screenshot tool

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
