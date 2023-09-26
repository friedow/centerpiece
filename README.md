# Hey 👋

If you are looking for my current (working) omnibox search setup, you can find it here: <https://github.com/friedow/dotfiles/tree/main/modules/launcher>. It is rofi based with a lot of custom plugins implemented in nushell, all configured in nix ;).

I recently experimented with onagre + pop launcher as a omnibox setup because rofi got really slow with an increasing amount of plugins. You can find these experiments here: <https://github.com/friedow/dotfiles/tree/main/modules/onagre>. This is onagre + pop-launcher + custom plugins in nushell, all configured in nix of course.

To address the elephant in the room: This repository is another experiment of implementing all of the above in rust 😅. I think this is my sixth attempt of getting this right, so please don't expect anything finished here soon 😋.

## Centerpiece

Centerpiece is a omnibox search for linux.
This project is currently in a very early state and tailored to my needs and daily workflows.

![Screenshot of the applications in its default state.](./screenshots/search-view.png)

### Features

- [x] switch windows (sway)
- [x] open applications (XDG based, scans for .desktop files)
- [ ] open brave in app mode for special bookmarks
- [x] open local git repositories (terminal, editor, git gui)
  - [ ] needs an index to increase speed
- [ ] open brave bookmarks
- [ ] switch wifi networks
- [ ] control audio devices
- [ ] display resource monitor (cpu, gpu, ram, disks)
- [ ] display battery state
- [x] display date and time
- [ ] display weather
- [ ] search brave history

### TODO
- [ ] nix build cache for this repo

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
