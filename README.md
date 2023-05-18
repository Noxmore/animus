# Animus
Framework independent lightweight rust animation library.

> NOTE: currently only f32s are able to be animated, this almost certainly will change in the future, but for now you'll have to work around it, or make a PR to help fix it yourself :P

# Usage
- Add it to your project with the terminal: `cargo add animus`.
- Create an instance of the `Animus` struct somewhere in your code. (e.g. before your main loop, or in your App struct) - example:
```rust
let animus = Animus::default();
```
- At the end of your update/frame function, call `anim.gc();` to cleanup unused animations.
- Now from your update/frame function, you can define and use a new animation:
```rust
use animus::prelude::*;

let animated_value = animus.anim("animation_name", 50., -50, 5., ease_in_out(3.));
                                   animation_id   start  end time   animator
```