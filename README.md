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
- [ ] open firefox in app mode for special bookmarks
- [x] open local git repositories (terminal, editor, git gui)
  - [x] needs an index to increase speed
- [ ] open brave bookmarks
- [ ] open firefox bookmarks
- [ ] convert between units (e.g. cm to inch)
- [ ] switch wifi networks
- [ ] control audio devices
  - [ ] select default microphone / speaker
  - [ ] control volume
- [ ] control mpd:
    - [ ] select playlist
    - [ ] add to playlist
    - [ ] play, pause, stop, next track, prev track
- [ ] run system commands (lock, sleep, restart, shutdown)
- [ ] list and run user defined scripts
- [ ] run command in terminal (commands are prefixed with `:`)
- [ ] unit converter (https://github.com/printfn/fend)
- [x] display resource monitor (cpu, gpu, ram, disks)
  - [ ] split those into seperate files
- [x] display battery state
- [x] display date and time
- [ ] display weather
- [ ] search brave history
- [ ] search firefox history

### TODO

- [x] nix build cache for this repo
- [x] use crane as a build tool
- [ ] nix module to configure systemd services and install app
- [ ] use a gif instead of a png to showcase app in readme

### Repository Structure

#### /client

Contains the graphical application and the plugin code that is needed during runtime. This is most of the plugin code which handles requests for searching and opening of entries.

#### /services

Computations for generating plugin entries can be time consuming. For example listing all git repositories entails searching your whole home directory for directories with the name `.git`. To avoid slowing down the graphical application during run time this directory contains code for small systemd services that write indices for plugins with time consuming queries.

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
