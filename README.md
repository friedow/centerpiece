# Hey 👋

If you are looking for my current (working) omnibox search setup, you can find it here: <https://github.com/friedow/dotfiles/tree/main/modules/launcher>. It is rofi based with a lot of custom plugins implemented in nushell, all configured in nix ;).

I recently experimented with onagre + pop launcher as a omnibox setup because rofi got really slow with an increasing amount of plugins. You can find these experiments here: <https://github.com/friedow/dotfiles/tree/main/modules/onagre>. This is onagre + pop-launcher + custom plugins in nushell, all configured in nix of course.

To address the elephant in the room: This repository is another experiment of implementing all of the above in rust 😅. I think this is my sixth attempt of getting this right, so please don't expect anything finished here soon 😋.

## Centerpiece

Centerpiece is a omnibox search for linux.
This project is currently in a very early state and tailored to my needs and daily workflows.

![Screenshot of the applications in its default state.](./screenshots/search-view.png)

## Features

- [ ] Switch between open windows (sway)
- [ ] Open Applications (XDG based)
- [ ] Open a development environment in local git repositories

### Development Setup

#### Build Environment

The `flake.nix` provides a ready-to-roll build environment usable with `nix develop`.

#### Building the Application

1. Run a new bash shell containing the build environment

   ```bash
   nix develop
   ```

2. Run the application

   ```bash
   cargo run
   ```
