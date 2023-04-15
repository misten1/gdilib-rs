# gdilib-rs

![crates.io](https://img.shields.io/crates/v/gdilib-rs.svg)

--------

A simple library that supplies functions for easy GDI manipualation of the desktop
using winapi

## Examples
```rust
// Applies a simple RGB glitch using BitBlt onto the desktop of the current user
unsafe {
    let my_desktop = effects::get_desktop();
    my_desktop.rgb_glitch();
    my_desktop.close(); // Cleanup
}
```

```rust
// Draws a red rectangle on the user's desktop
unsafe {
    let my_desktop = effects::get_desktop();
    let mut rectangle = utils::create_rect(10, 10, 100, 100); // x, y, width, height
    my_desktop.fill_rect(&mut rectangle, utils::rgb_to_colorref(255, 0, 0));
    my_desktop.close();
}
```

## Advanced effects

SingleEffect allows you to execute one effect defined by a enum in ```complexeffect::EFFECTS```

```rust
let desktop = effects::get_desktop();
let my_effect = SingleEffect::new(FLIPH); // Flip horizontally

unsafe {
    my_effect.execute(&desktop);
    desktop.close();
}
```

You can chain effects together easily using ```EffectChain```

```rust
let desktop = effects::get_desktop();

// 10ms delay inbetween effects
let my_effect = EffectChain::new(10, vec![FLIPH, RGB, MELT, FLIPV]); // Easily chain effects together

unsafe {
    // Execute all effects
    my_effect.execute(&desktop);
    // Cleanup
    desktop.close();
}
```

## todo:
- Add more effects
- Make completely custom effects possible

-----------------

## Have fun!
