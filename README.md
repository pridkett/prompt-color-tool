prompt-color-tool
=================

A swiss army knife of functions that help with prompt colors

Patrick Wagstrom &lt;160672+pridkett@users.noreply.github.com&gt;

November 2024

Overview
--------

I've been working on various ways that I can have consistent colors across my machines. And what I mean by consistent is not so much to have the same colors on each machine, but to have different colors on each machine, but have them sync'd up between applications like powerline-go, tmux, and iTerm.

I was able to do this with a lot of shell scripting, but in the end, it just kinda turned into a mess. So, I created this program to provide a single consistent reference point for me on how to calculate the colors, either as one of the 256 xterm colors or as hex, and use them in various programs.

It might be easier to show how this can work. In the screencast below, you can see me using iTerm to connect a number of different machines and running `tmux` on each of those machines. In each case the color of the window title bar, the hostname in the prompt, and the color of the status bar in `tmux` are synchronized across the applications. This provides a nice visual indicator of which machine I'm working with.

<p align="center">
    <img src="docs/demo.gif" alt="Demo">
</p>

### Usage

```
prompt-color-tool [OPTIONS] [hostname]
```

If `hostname` is not provided on the command line, the program will grab the system short hostname and use that.

* `--theme <theme>`   The theme to use for color calculation [default: `default` - other values are `low-contrast`, `solarized-light16`, and `solarized-dark16`]
* `--noenv`           Ignore the environment variables `PLGO_HOSTNAMEBG` and `PLGO_HOSTNAMEFG`. 
* `--bgcolor <color>` A fixed color to use as the background color. This must be in the range of 0-255. This is for the use case when you know that the hash of the name is going to generate an unpleasant color and you want to override, but still calculate the foreground color or have an easy way to get it as hex, etc.
* `--fgcolor <color>` A fixed color to use as the foreground color. This must be in the range of 0-255. This is for the use case when you know that the hash of the name is going to generate an unpleasant color and you want to override, but still calculate the background color or have an easy way to get it as hex, etc.
* `--hex`             Output colors in hexadecimal format
* Mutually Exclusive Output Control Options
  * `-v`, `--verbose`     Print verbose output. There's really not much here, but it will tell you which number is which.
  * `-f`, `--fgonly`      Only output the foreground color. Useful if you don't want to do other shell scripting to separate the numbers.
  * `-b`, `--bgonly`      Only output the background color. Useful if you don't want to do other shell scripting to separate the numbers.
  * `--iterm` Rather than printing the colors as normal, send the ANSI escape codes to iTerm can set the color of the tab.

Note that there are several ways that the colors can be specified. The precedence order is as follows:

1. If `--bgcolor` or `--fgcolor` are specified, those will be used.
2. If the environment variables `PLGO_HOSTNAMEBG` or `PLGO_HOSTNAMEFG` are set and `noenv` is not true, those will be used.
3. Calculate the varible. Background color is calculated first and based off the hostname to color mapping algorithm below. Foreground color is calculated after and based of a static mapping of colors.

Note, the precedence can be mixed and matched. For example, you can define `--bgcolor` and still have it calculate the foreground color. Likewise, you can set a value to the environment variable `PLGO_HOSTNAMEFG` and the background color will still be calculated independent.

### Hostname to Color Mapping Algorithm

Because I use [powerline-go][powerline-go], I wanted to ensure that the [automatically selected colors were the same as those used in the powerline-go algorithm][powerline-go-algorithm].

1. Get just the local hostname of the machine (everything before the first period)
2. Calculate the md5 hash of the hostname
3. Take the first byte of the hash and run modulo 128 on it to get a number between 0 and 127
4. Use the number to select a color from the xterm 256 color palette

Application Integration
-----------------------

### powerline-go

This is based off the way that powerline-go already handles the colorization and even uses the same environment variables. You can override the colors by setting `PLGO_HOSTNAMEFG` and `PLGO_HOSTNAMEBG` and they will work for all the applications.

### tmux

In your `.tmux.conf` file, you can set the colors of the status bar to be the same as the colors of the hostname. Here's an example of how you can do that:

```tmux
#
# set the color of the status bar according to the hostname of the machine
#

%if #{?PLGO_HOSTNAMEBG,1,0}
    set -g status-bg colour$PLGO_HOSTNAMEBG
%else
    run "tmux set -g status-bg $(printf colour%s $(~/.cargo/bin/prompt-color-tool --bgonly))"
%endif

%if #{?PLGO_HOSTNAMEFG,1,0}
    set -g status-fg colour$PLGO_HOSTNAMEFG
%else
    run "tmux set -g status-fg $(printf colour%s $(~/.cargo/bin/prompt-color-tool --fgonly))"
%endif
```

### iTerm (with fish shell)

This is a two step process. First, we need to determine if we should even try to output the escape codes. In theory, we should be fine without this, but this is easiest way to be assured is be checking if it is interactive and not in a terminal multiplexer. Then, we attach the hook to the postexec event in fish to call the program to set the tab color. This has the nice side effect of it will set the color whenever the shell starts and also when commands finish. So the tab color will be set when you exit an SSH session too.

Of note, not all systems pass through the escape codes necessary for this to work. Notably, `mosh` does not pass through the custom iTerm escape codes required for this feature.

```fish
function iterm2_should_integrate
    if begin;
	status --is-interactive;
	and test "$ITERM_ENABLE_SHELL_INTEGRATION_WITH_TMUX""$TERM" != screen;
	and test "$ITERM_ENABLE_SHELL_INTEGRATION_WITH_TMUX""$TERM" != screen-256color;
	and test "$ITERM_ENABLE_SHELL_INTEGRATION_WITH_TMUX""$TERM" != tmux-256color;
	and test "$TERM" != dumb;
        and test "$TERM" != linux; end
	return 0
    end
    return 1
end

function set_iterm_tab_color --on-event fish_postexec
    ~/.cargo/bin/prompt-color-tool --iterm
end

# Call the function to set the tab color when the shell starts
if iterm2_should_integrate
    set_iterm_tab_color
end
```

Compilation Notes
-----------------

Native compilation should be straightforward:

```bash
cargo build --release
cargo install --path .
```

If `~/.cargo/bin` is in your path then you'll now have the `prompt-color-tool` command available to you.

### Cross Compilation
This section is more for myself, so I remember, but other people might find it interesting. After trying to muck around with installing various cross-architecture toolchains on my Mac, and getting some to work and some failing miserably, I think the easiest way to do static cross-compilation is just to use docker. Here's the commands that I use.

For x86_64 Linux:
```bash
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:x86_64-musl cargo build --release
```

For aarch64 Linux (i.e. Raspberry Pi running 64 bit OS):
```bash
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:aarch64-musl bash -c "rustup target add aarch64-unknown-linux-musl && cargo build --release"
```

For armv7 Linux (i.e. Raspberry Pi running 32 bit OS):
```bash
docker run --rm -it -v "$(pwd)":/home/rust/src messense/rust-musl-cross:armv7-musleabihf bash -c "rustup target add armv7-unknown-linux-musleabihf && cargo build --release"
```

For aarch64 MacOS there's an issue with the builder image that I'm using not being able to strip Mach-O binaries. The image has `llvm-strip` installed, but there's not an easy (any?) way to get `rustc` to call it. This little hack takes advantage of the fact that /root/.cargo/bin comes first in the path, so we do something bad and just symlink it there. This hack makes it so `rustc` calls `llvm-strip` without really knowing it.

```bash
docker run --rm -it -v "$(pwd)":/root/src --workdir /root/src joseluisq/rust-linux-darwin-builder:1.82.0 \
sh -c "ln -s /usr/bin/llvm-strip /root/.cargo/bin/strip && cargo build --release --target aarch64-apple-darwin"
```


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
