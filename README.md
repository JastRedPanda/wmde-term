# WMDE Terminal

Terminal emulator for the WMDE desktop (COSMIC-based).

WMDE Terminal is part of the [WMDE desktop](https://wmde.fun) and is a fork of
[pop-os/cosmic-term](https://github.com/pop-os/cosmic-term) by System76, rebranded for WMDE.
Original authorship and copyright are retained.

It is built on [alacritty\_terminal](https://docs.rs/alacritty_terminal) from the
[alacritty](https://github.com/alacritty/alacritty) project, and provides bidirectional
rendering and ligatures through a custom renderer based on
[cosmic-text](https://github.com/pop-os/cosmic-text).

The `wgpu` feature, enabled by default, supports GPU rendering using `glyphon`
and `wgpu`. If `wgpu` is not enabled or fails to initialize, then rendering falls
back to using `softbuffer` and `tiny-skia`.

## Color Schemes

Custom color schemes can be imported from the `View -> Color schemes...` menu item.
You can find templates for color schemes in the [color-schemes](color-schemes) folder.
The built-in schemes are `WMDE Dark` and `WMDE Light`; a config that still names the
pre-rebrand `COSMIC Dark`/`COSMIC Light` is migrated to them on startup.

## Build (Arch Linux)

A `PKGBUILD` is provided:

```sh
makepkg -si
```
