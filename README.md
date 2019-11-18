# rust-pushrod

## Project Description

[![Build Status](https://travis-ci.org/KenSuenobu/rust-pushrod.svg?branch=master)](https://travis-ci.org/KenSuenobu/rust-pushrod)
[![](https://img.shields.io/crates/d/rust-pushrod.svg)](https://crates.io/crates/rust-pushrod)
[![docs.rs for rust-pushrod](https://docs.rs/rust-pushrod/badge.svg)](https://docs.rs/rust-pushrod)

**Cross Platform UI Widget Library for Rust that uses SDL2.**

Draws inspiration from lots of GUI libraries.

If you like this library, [please consider donating to this project!](https://www.patreon.com/KenSuenobu)

## Philosophy

The reason I created this library instead of extending another library was that
I wanted to keep these specific design ideas in mind:

- Maintainable with little effort
- Easily extensible
- Lightweight enough to run on minimalist hardware
- **Easy to use and understand**

These design ideas are critical.  **Keep it simple.  Keep it stupid simple.**

## (Ever Evolving) Screenshot of Sample

[![](docs/sample-0.3.7.png)](docs/sample-0.3.7.png)

## 0.4.x Status

Soon to follow will be:

- [x] Support for Layouts
  - [ ] Dialog or Form Layout
  - [ ] Grid Layout
- [ ] Menu Bar Widget
- [ ] Simple Popup Menu Widget
- [ ] Drop Down Menu Widget
- [ ] Drop Down Menu Button Widget
- [ ] Grid Widget
- [ ] Toolbox Widget
- [ ] Tab Widget
- [ ] Group Box Widget
- [ ] Split Container Widget
- [ ] Slider Widget
- [ ] Viewport Widget that adjusts a viewable area with scrolling

## Additional Items

- [x] Support for Layouts
  - [x] Horizontal Layout
  - [x] Vertical Layout
- [x] Port engine from Piston to SDL2
- [x] Text Widget
  - [x] OpenSans Font Included
- [x] Image Widget
- [x] Progress Bar Widget
- [x] Timer Widget
- [x] Push Button Widget
- [x] Image Button Widget
- [x] Toggle Button Widget
- [x] Checkbox Widget
- [x] Fix build so that it builds on TravisCI for both OSes properly
- [x] New traits for optimization:
  - [x] `Drawable`
  - [x] `InjectableSystemEvents`
  - [x] `InjectableCustomEvents`
- [x] Update Piston to the latest release
- [x] Fix hidden object traversal in main draw loop
- [x] Pass in event system trait so that `Widget`s have access to the `Widgets` from the `WidgetStore`
- [x] Add helper methods to access widgets by name and invalidate via Widget vector
- [x] Add Callbacks:
  - [x] On Click without number of clicks
  - [x] Mouse Moved
  - [x] Widget Toggled
  - [x] Timer Tick
  - [x] Mouse Button Down
  - [x] Mouse Button Up
  - [x] Mouse Entered
  - [x] Mouse Exited
- [x] Get rid of Point and Size, use array value types instead
- [x] Horizontal Layout sample application
- [x] Vertical Layout sample application
- [x] Update documentation
- [x] Layout Manager optimizations (call `do_update` at the end of layout)
- [ ] Modify Event system so that it is a single trait
- [ ] Fix all `Widget`s so that they draw to a texture
- [ ] Fix Invalidation such that it walks the children so that drawing is done in order
- [ ] Widget offset updates (adding a child origin based on parent)
- [ ] Optimizations to screen invalidation
- [ ] Optimizations to drawing (every object is a 3D texture, not just blitting to a master texture)
- [ ] `PageWidget`: page controller to contain and swap displays of different pages of containers
- [ ] `GridLayoutContainer`: layout objects equally sized in a grid
- [ ] `ToolboxWidget`: layout that displays clickable images and captioned text
- [ ] `HorizontalSpaceWidget`: horizontal spacer for layout containers
- [ ] `VerticalSpaceWidget`: vertical spacer for layout containers
- [ ] `DropdownMenuWidget`: displays a dropdown menu with selectable items in a list
- [ ] `SplitContainerWidget`: splits two displays horizontally or vertically by a resizable spacer
- [ ] `GridWidget`: displays a grid (either by lines or dots) evenly spaced by a grid snap offset
- [ ] `TabWidget`: displays a series of tabs that can be used with the `PageWidget` to control page switching
- [ ] `PopupMenuWidget`: displays a dropdown menu anywhere on the screen based on the mouse offset
- [ ] `Viewport`: container that is larger than the screen that can be repositioned
- [ ] `SliderWidget`: displays a slideable box in a bounding box, allowing for content to be scrolled
- [ ] `MenuBar`: creates a menu bar at the top of the window that can be used to interact with the app
- [ ] Layout for dialog boxes and/or form displays
- [ ] Modal Alert box either by a new window or by a static modal dialog box that disables the top-level container
- [ ] (De)serialization library for widgets to file
- [ ] Modification of main application to use (de)serialization for layout

Please [see here](https://github.com/KenSuenobu/rust-pushrod/milestone/5) for more details on issues.

## Prerequisites for Pushrod

Pushrod only requires:

| Library | Version |
| ------- | ------- |
| SDL2    | 0.32 |

### Ubuntu

```bash
sudo apt install libsdl2-dev libsdl2-ttf-dev libsdl2-image-dev
```

### Mac OS X

```bash
brew update
brew upgrade
brew install ruby
brew install sdl2 sdl2_image sdl2_ttf
```
