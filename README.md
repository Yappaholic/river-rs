### Library for writing RiverWM Configuration in Rust!

This is just a proof of concept and my attempt at better understanding `Rust`.

Currently Work In Progress.

## Why?

Well, why not? After all, why not build something unique and fun.

On a more serious note, writing configuration for `RiverWM` in programming languages
teaches a lot about language's basic operations, loops, typing, working with strings,
creating compound objects and tooling. That's why I would recommend _you too_ to try
and build a simple library for configuring `RiverWM` in any language you want
(`C`, `Zig`, `Haskell`, ~Java?~, _whatever!_).

## Current status

Right now you can describe and apply custom keybindings in a procedural way,
when you apply keybinds one by one, depending on wether you want to change the modifier keys,
and then apply config in the end.

# TODO:

- [x] Write function to generate tag keybinds
- [x] Write autospawn function (You can use `spawn` function by now)
- [x] Fix setting xkb keyboard layout
- [ ] Write support for complex spawn arguments
- [ ] Make writing custom rules work
