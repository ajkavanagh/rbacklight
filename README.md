# rbacklight: Adjust intel backlight in sysfs

*sysfs backlight control of an intel GPU laptop where xrandr xbacklight
doesn't work.*

Inspired by: https://github.com/szekelyszilv/ybacklight

I needed a backlight control for Ubuntu 18.04 in non Gnome sessions,
specifically i3 and xMonad.
[xbacklight](https://github.com/tcatm/xbacklight) doesn't work because it
uses the xrandr extension, and this isn't working on Bionic.  ybacklight
does work, but it's written in C.  That's not a *bad* thing, per se, but
I'd been looking for the opportunity to write some Rust, and so rbacklight
happened.

# Building

rbacklight needs a rust toolchain to build it.
[rustup.rs](https://rustup.rs/) is probably the easiest way to get it.
The stable channel is the one to use.

## Steps

1. Clone the repository.
2. `cargo build --release`
3. Copy `./target/release/rbacklight` to somewhere in the `$PATH`.  I use
   `/usr/local/bin/rbacklight`.
4. Change ownership to root: `chown root.root rbacklight`
5. Make it *setuid*: `chmod u+s rbacklight`.  rbacklight needs sudo
   permissions to write to the backlight sysfs file, but you need to run
   it as a user.  If you're uncomfortable running this code as root, the
   please look elsewhere.
6. You should now be able to set your backlight with: `rbacklight set 50`.
   This should set it to 50%.

# Using

```bash
rbacklight <command> <value-in-percent>
```

rbacklight has the following commands:

- *get*: get the current backlight percentage (default command)
- *set <value>*: set the backlight to <value> percent.
- *inc <value>*: increment the backlight by <value> percent.
- *dec <value>*: decrement the backlight by <value> percent.
- *help*: print a very short help message to `stderr`

Running *rbacklight* on it's own should print the current backlight
setting in percent.  The default increment and decrement is 10% and the
default *set* is 70%.
