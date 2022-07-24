# clers

cle**rs** is a reimplementation of [keynav](https://www.semicomplete.com/projects/keynav/) in Rust for Wayland compositors.

## Requirements

* A Wayland compositor
* [wlrctl](https://git.sr.ht/~brocellous/wlrctl)

## Building/Installing

Build with cargo.

```bash
$ cargo build --release
```

Copy or syslink to somewhere in your `$PATH`.

```bash
$  cp target/release/clers /usr/local/bin/
# or syslink
$  ln -s ${FULL_PATH_TO_BUILD} ${FULL_PATH_TO_TARGET}
```

Bind a key to execute `clers`. For example, with [Sway](https://swaywm.org/), you can add this line to your Sway config to run clers with `mod+c`.

```
bindsym $mod+c exec clers
```
