// Pushrod Rendering Library
// Widget Caching Library
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::RefCell;

use crate::render::widget::Widget;
use crate::render::widget_config::{CONFIG_ORIGIN, CONFIG_SIZE};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

/// This is a container that stores information about a `Widget` that will be drawn on the screen.
/// It stores the `Widget` object, the actual point of origin inside the `Window` (as a `Vec<i32>`
/// of X and Y points), the parent ID of this `Widget`, if it is being added as a child.
pub struct WidgetContainer {
    pub widget: RefCell<Box<dyn Widget>>,
    widget_name: String,
    pub origin: Vec<i32>,
    widget_id: i32,
    parent_id: i32,
}

/// This is the `WidgetContainer` object that stores a `Widget` and its accompanying information:
/// its name, point of origin, and parent ID.
impl WidgetContainer {
    /// Creates a new `WidgetContainer` storage object.
    pub fn new(
        widget: Box<dyn Widget>,
        widget_name: String,
        origin: Vec<i32>,
        widget_id: i32,
        parent_id: i32,
    ) -> Self {
        Self {
            widget: RefCell::new(widget),
            widget_name: widget_name.clone(),
            origin: origin.clone(),
            widget_id,
            parent_id,
        }
    }

    /// Retrieves the name of this `Widget`.
    pub fn get_widget_name(&self) -> String {
        self.widget_name.clone()
    }

    /// Retrieves the numeric ID of this `Widget`.
    pub fn get_widget_id(&self) -> i32 {
        self.widget_id
    }

    /// Retrieves the numeric ID of the parent that this `Widget` refers to.  A `0` indicates
    /// no parent is assigned.
    pub fn get_parent_id(&self) -> i32 {
        self.parent_id
    }
}

/// This is the `WidgetCache` struct, which contains a list of `Widget`s that are managed by the Pushrod
/// `Engine`.  `Widget` IDs are automatically generated by the `WidgetCache`, which automatically
/// assigns the `Widget` ID at the time it's added to the cache.  Parent IDs must already exist,
/// otherwise, an error is thrown at the time the `Widget` is attempted to be added.  `Widget` IDs
/// always start at 1.
#[derive(Default)]
pub struct WidgetCache {
    cache: Vec<WidgetContainer>,
}

/// This is the `WidgetCache` implementation.  This cache object manages the `Widget` list for use by the
/// Pushrod `Engine`.
impl WidgetCache {
    pub fn new() -> Self {
        Self { cache: Vec::new() }
    }

    /// This adds a `Widget` to the render list.  It requires that the `Widget` being added is in a `Box`,
    /// along with a `widget_name`.  Returns the ID of the `Widget` that was added.  Use this ID if
    /// you plan on adding further `Widget`s, with this `Widget` as the parent.  The point of
    /// `origin` (extracted from the `Widget`'s position at creation time) is its physical location
    /// inside the `Window`.
    pub fn add_widget(&mut self, mut widget: Box<dyn Widget>, widget_name: String) -> i32 {
        let origin = widget.get_config().get_point(CONFIG_ORIGIN);
        let widget_id = self.cache.len();

        self.cache.push(WidgetContainer::new(
            widget,
            widget_name.clone(),
            origin,
            widget_id as i32,
            0,
        ));

        (self.cache.len() - 1) as i32
    }

    /// This locates the ID of a `Widget` at a given `x` and `y` coordinate.  If a `Widget` could not
    /// be found, the top-level `Widget` (id 0) is returned.  This function returns the top-most
    /// visible `Widget` id.
    pub fn find_widget(&mut self, x: i32, y: i32) -> i32 {
        let mut found_widget_id: i32 = 0;

        for i in 0..self.cache.len() {
            if !self.is_hidden(i as i32) {
                let start_x: i32 = self.cache[i]
                    .widget
                    .borrow_mut()
                    .get_config()
                    .get_point(CONFIG_ORIGIN)[0];
                let start_y: i32 = self.cache[i]
                    .widget
                    .borrow_mut()
                    .get_config()
                    .get_point(CONFIG_ORIGIN)[1];
                let end_x: i32 = start_x
                    + (self.cache[i]
                        .widget
                        .borrow_mut()
                        .get_config()
                        .get_size(CONFIG_SIZE)[0] as i32);
                let end_y: i32 = start_y
                    + (self.cache[i]
                        .widget
                        .borrow_mut()
                        .get_config()
                        .get_size(CONFIG_SIZE)[1] as i32);

                if x >= start_x && x <= end_x && y >= start_y && y <= end_y {
                    found_widget_id = i as i32;
                }
            }
        }

        found_widget_id
    }

    /// Returns a `WidgetContainer` object by its ID.  This is the same `Widget` ID that is returned
    /// when using the `add_widget` function.  There are no bounds checks here, so if the ID does not
    /// exist, it will throw an exception at runtime.  Be careful: it's better to use the
    /// `get_container_by_name` function to avoid this.
    pub fn get_container_by_id(&mut self, id: i32) -> &mut WidgetContainer {
        &mut self.cache[id as usize]
    }

    /// Returns a `WidgetContainer` object by the name of the `Widget`.  If the `WidgetContainer`
    /// cannot find the `Widget` by the `name` specified, the top-level `Widget` is returned for
    /// safety.
    pub fn get_container_by_name(&mut self, name: String) -> &mut WidgetContainer {
        let cache_size = self.cache.len();

        for i in 0..cache_size {
            if self.cache[i].get_widget_name() == name {
                return self.get_container_by_id(i as i32);
            }
        }

        self.get_container_by_id(0 as i32)
    }

    /// This function calls the `button_clicked` callback for the `Widget` specified by `widget_id`.
    pub fn button_clicked(&mut self, widget_id: i32, button: u8, clicks: u8, state: bool) {
        if !self.is_hidden(widget_id) && self.is_enabled(widget_id) {
            self.cache[widget_id as usize]
                .widget
                .borrow_mut()
                .button_clicked(&self.cache, button, clicks, state);
        }
    }

    /// This function calls the `mouse_moved` callback for the `Widget` specified by `widget_id`.
    pub fn mouse_moved(&mut self, widget_id: i32, points: Vec<i32>) {
        if !self.is_hidden(widget_id) && self.is_enabled(widget_id) {
            self.cache[widget_id as usize]
                .widget
                .borrow_mut()
                .mouse_moved(&self.cache, points);
        }
    }

    /// This function calls the `mouse_scrolled` callback for the `Widget` specified by `widget_id`.
    pub fn mouse_scrolled(&mut self, widget_id: i32, points: Vec<i32>) {
        if !self.is_hidden(widget_id) && self.is_enabled(widget_id) {
            self.cache[widget_id as usize]
                .widget
                .borrow_mut()
                .mouse_scrolled(&self.cache, points);
        }
    }

    /// This function calls the `mouse_exited` callback for the `Widget` specified by `widget_id`.
    pub fn mouse_exited(&mut self, widget_id: i32) {
        if !self.is_hidden(widget_id) && self.is_enabled(widget_id) {
            self.cache[widget_id as usize]
                .widget
                .borrow_mut()
                .mouse_exited(&self.cache);
        }
    }

    /// This function calls the `mouse_entered` callback for the `Widget` specified by `widget_id`.
    pub fn mouse_entered(&mut self, widget_id: i32) {
        if !self.is_hidden(widget_id) && self.is_enabled(widget_id) {
            self.cache[widget_id as usize]
                .widget
                .borrow_mut()
                .mouse_entered(&self.cache);
        }
    }

    /// This function calls the `tick` method on all registered `Widget`s in the cache.  The purpose
    /// for the `tick` is to indicate that a drawing loop is about to occur, and the `Widget` can
    /// update itself as necessary beforehand.
    pub fn tick(&mut self) {
        let cache_size = self.cache.len();

        for i in 0..cache_size {
            if !self.is_hidden(i as i32) {
                self.cache[i].widget.borrow_mut().tick(&self.cache);
            }
        }
    }

    /// This function performs the draw loop for all of the `Widget`s stored in the `cache`.  Each
    /// `Widget` receives a mutable reference to the `Canvas` so that the `Widget` can be drawn on
    /// the screen during the draw loop of the `Engine`.  This `draw_loop` function automatically
    /// clips the screen area so that the `Widget` cannot draw outside of its bounds.
    pub fn draw_loop(&mut self, canvas: &mut Canvas<Window>) {
        let cache_size = self.cache.len();

        for i in 0..cache_size {
            if self.cache[i].widget.borrow_mut().get_config().invalidated() {
                self.draw(0, canvas);

                return;
            }
        }
    }

    // Private functions

    fn get_children_of(&mut self, widget_id: i32) -> Vec<i32> {
        self.cache
            .iter()
            .filter(|x| x.parent_id == widget_id)
            .map(|x| x.widget_id)
            .collect()
    }

    fn draw(&mut self, widget_id: i32, c: &mut Canvas<Window>) {
        let parents_of_widget = self.get_children_of(widget_id);

        if parents_of_widget.is_empty() {
            return;
        }

        let top_level_rect = self.cache[0].widget.borrow_mut().get_drawing_area();
        let mut needs_present = false;

        for paint_id in &parents_of_widget {
            let paint_widget = &mut self.cache[*paint_id as usize];
            let is_hidden = paint_widget.widget.borrow_mut().get_config().is_hidden();
            let is_enabled = paint_widget.widget.borrow_mut().get_config().is_enabled();
            let is_invalidated = paint_widget.widget.borrow_mut().get_config().invalidated();
            let widget_x = paint_widget.widget.borrow_mut().get_config().to_x(0);
            let widget_y = paint_widget.widget.borrow_mut().get_config().to_y(0);
            let widget_w = paint_widget
                .widget
                .borrow_mut()
                .get_config()
                .get_size(CONFIG_SIZE)[0];
            let widget_h = paint_widget
                .widget
                .borrow_mut()
                .get_config()
                .get_size(CONFIG_SIZE)[1];

            eprintln!(
                "Widget redraw: id={:?} hidden={} invalidated={}",
                paint_id, is_hidden, is_invalidated
            );

            if !is_hidden && is_invalidated {
                c.set_clip_rect(paint_widget.widget.borrow_mut().get_drawing_area());
                paint_widget.widget.borrow_mut().draw(c);
                paint_widget
                    .widget
                    .borrow_mut()
                    .get_config()
                    .set_invalidate(false);
                c.set_clip_rect(top_level_rect);

                needs_present = true;
            }

            if *paint_id != widget_id {
                self.draw(*paint_id, c);
            }

            if !is_enabled {
                c.set_draw_color(Color::RGBA(0, 0, 0, 128));
                c.draw_rect(Rect::new(widget_x, widget_y, widget_w, widget_h))
                    .unwrap();
            }
        }

        if needs_present {
            eprintln!("Presenting canvas.");
            c.present();
        }
    }

    fn is_hidden(&self, widget_id: i32) -> bool {
        self.cache[widget_id as usize]
            .widget
            .borrow_mut()
            .get_config()
            .is_hidden()
    }

    fn is_enabled(&self, widget_id: i32) -> bool {
        self.cache[widget_id as usize]
            .widget
            .borrow_mut()
            .get_config()
            .is_enabled()
    }
}
