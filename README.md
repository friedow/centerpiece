# Centerpiece

_Your trusty omnibox search._
This project is currently in a very early state and tailored to my needs and daily workflows.

![Screenshot of the applications in its default state.](./screenshots/search-view.png)

## Features

- switch between windows in [sway](https://swaywm.org/)
- open applications
- open [brave browser](https://brave.com/) in app-mode for special bookmarks
- open local git repositories in a terminal, an editor and a git gui
- open [brave browser](https://brave.com/) bookmarks in a new tab
- run commands to lock, sleep, restart or shutdown the system
- display information about cpu, gpu, ram and disks
- display battery state
- display date and time

## Repository Structure

### /client

Contains the graphical application and the plugin code that is needed during runtime. This is most of the plugin code which handles requests for searching and opening of entries.

### /services

Computations for generating plugin entries can be time consuming. For example listing all git repositories entails searching your whole home directory for directories with the name `.git`. To avoid slowing down the graphical application during run time this directory contains code for small systemd services that write indices for plugins with time consuming queries.

## Development Setup

### Build Environment

The `flake.nix` provides a ready-to-roll build environment usable with `nix develop`.

### Building the Application

1. Run a new bash shell containing the build environment

   ```bash
   nix develop
   ```

2. Run the application

   ```bash
   cargo run
   ```
