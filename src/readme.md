# gdilib-rs

A simple library that supplies functions for easy GDI manipualation of the desktop
using winapi

## Examples
```rust
// Applies a simple RGB glitch using BitBlt onto the desktop of the current user
unsafe {
    let my_desktop = effects::get_desktop();
    my_desktop.rgb_glitch();
}
```

```rust
// Draws a red rectangle on the user's desktop
unsafe {
    let my_desktop = effects::get_desktop();
    let mut rectangle = utils::create_rect(10, 10, 100, 100); // x, y, width, height
    my_desktop.fill_rect(&mut rectangle, utils::rgb_to_colorref(255, 0, 0));
}
```

## Have fun!