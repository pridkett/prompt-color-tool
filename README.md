prompt_color_tool
=================

A swiss army knife of functions that help with prompt colors

Patrick Wagstrom <160672+pridkett@users.noreply.github.com>

November 2024

Overview
--------

### Hostname to Color Mapping

Because I use [powerline-go][powerline-go], I wanted to ensure that the [automatically selected colors were the same as those used in the powerline-go algorithm][powerline-go-algorithm].

1. Get just the local hostname of the machine (everything before the first period)
2. Calculate the md5 hash of the hostname
3. Take the first byte of the hash and run modulo 128 on it to get a number between 0 and 127
4. Use the number to select a color from the xterm 256 color palette


Caveats
-------

I've never really programmed in Rust before, so I'm sure there are some things that are strange in this program. Feel free to let me know where there are things that are not idiomatic or where I'm using outdated patterns or where I'm just doing things strangely.

I mainly did this in Rust as an excuse to learn Rust and because otherwise it probably would've be in golang, but that would've resulted in a binary that was 10-20x the size. That gets chunky if it's being called multiple times when generating each prompt.

License
-------

Copyright (c) 2024 Patrick Wagstrom

Licensed under terms of the MIT License (see LICENSE file)

[powerline-go]: https://github.com/justjanne/powerline-go/
[powerline-go-algorithm]: https://github.com/justjanne/powerline-go/blob/059f7f230760f8800307b3ae632c6cc6ca3f81d4/segment-hostname.go#L42-L44
