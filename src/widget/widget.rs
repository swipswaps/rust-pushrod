// Widget Base Definition
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

use opengl_graphics::GlGraphics;
use piston_window::*;

use crate::core::point::*;
use crate::widget::config::*;

/// Implementable trait that is used by every `Widget`.  These are the public methods,
/// and a function _may_ override them.
///
/// You _must_ implement the following methods:
///
/// - get_config
/// - mouse_entered
/// - mouse_exited
/// - mouse_scrolled
///
/// You _should_ override `draw`, but you are not required to.
///
/// If you want a blank base widget, refer to the `BaseWidget`, which will create a
/// base widget that paints the contents of its bounds with whatever color has been
/// specified with `set_color`.
pub trait Widget {
    /// Retrieves the configuration HashMap that stores the configuration list of settings
    /// for this widget.
    ///
    /// To implement this, the following code could be used in your object's structure:
    ///
    /// ```
    /// # use pushrod::widget::widget::*;
    /// # use pushrod::widget::config::*;
    /// struct MyWidget {
    ///   config: Configurable,
    /// }
    ///
    /// impl MyWidget {
    ///   fn new() -> Self {
    ///     Self {
    ///       config: Configurable::new(),
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// And in the overridden function for get_config in your implementation, use:
    ///
    /// ```
    /// # use pushrod::widget::widget::*;
    /// # use pushrod::widget::config::*;
    /// # use pushrod::core::point::Point;
    /// struct MyWidget {
    ///   config: Configurable,
    /// }
    ///
    /// impl Widget for MyWidget {
    ///   fn config(&mut self) -> &mut Configurable {
    ///     &mut self.config
    ///   }
    ///
    ///  fn mouse_entered(&mut self, widget_id: i32) {}
    ///  fn mouse_exited(&mut self, widget_id: i32) {}
    ///  fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {}
    /// }
    /// ```
    ///
    /// This uses a `RefCell`, since configurations require a mutable reference to the HashMap
    /// that stores the configs.
    fn config(&mut self) -> &mut Configurable;

    /// Indicates that a widget needs to be redrawn/refreshed.
    fn invalidate(&mut self) {
        self.config()
            .set(CONFIG_INVALIDATE, WidgetConfig::Invalidate {});
    }

    /// Clears the invalidation flag.
    fn clear_invalidate(&mut self) {
        self.config().remove(CONFIG_INVALIDATE);
    }

    /// Checks to see whether or not the widget needs to be redrawn/refreshed.
    fn is_invalidated(&mut self) -> bool {
        self.config().contains_key(CONFIG_INVALIDATE)
    }

    /// Sets the `Point` of origin for this widget, given the X and Y origin points.  Invalidates the widget afterward.
    fn set_origin(&mut self, x: i32, y: i32) {
        self.config().set(
            CONFIG_ORIGIN,
            WidgetConfig::Origin {
                point: Point { x, y },
            },
        );
        self.invalidate();
    }

    /// Retrieves the `Point` of origin for this object.
    /// Defaults to origin (0, 0) if not set.
    fn get_origin(&mut self) -> Point {
        match self.config().get(CONFIG_ORIGIN) {
            Some(WidgetConfig::Origin { ref point }) => point.clone(),
            None => make_origin_point(),
            _ => make_origin_point(),
        }
    }

    /// Sets the `Size` for this widget, given a width and height.  Invalidates the widget afterward.
    fn set_size(&mut self, w: i32, h: i32) {
        self.config().set(
            CONFIG_SIZE,
            WidgetConfig::Size {
                size: crate::core::point::Size { w, h },
            },
        );
        self.invalidate();
    }

    /// Retrieves the `Size` bounds for this widget.
    /// Defaults to size (0, 0) if not set.
    fn get_size(&mut self) -> crate::core::point::Size {
        match self.config().get(CONFIG_SIZE) {
            Some(WidgetConfig::Size { ref size }) => size.clone(),
            _ => make_unsized(),
        }
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.config()
            .set(CONFIG_COLOR, WidgetConfig::Color { color });
        self.invalidate();
    }

    /// Retrieves the color of this widget.
    /// Defaults to white color `[1.0; 4]` if not set.
    fn get_color(&mut self) -> types::Color {
        if self.config().contains_key(CONFIG_COLOR) {
            match self.config().get(CONFIG_COLOR) {
                Some(WidgetConfig::Color { ref color }) => [color[0], color[1], color[2], color[3]],
                None => [1.0; 4],
                _ => [1.0; 4],
            }
        } else {
            [1.0; 4]
        }
    }

    fn set_autoclip(&mut self, clip: bool) {
        self.config()
            .set(CONFIG_AUTOCLIP, WidgetConfig::Autoclip { clip });
        self.invalidate();
    }

    fn get_autoclip(&mut self) -> bool {
        match self.config().get(CONFIG_AUTOCLIP) {
            Some(WidgetConfig::Autoclip { ref clip }) => clip.clone(),
            _ => false,
        }
    }

    // Events

    /// Called when a mouse enters the bounds of the widget.  Includes the widget ID.
    fn mouse_entered(&mut self, widget_id: i32);

    /// Called when a mouse exits the bounds of the widget.  Includes the widget ID.
    fn mouse_exited(&mut self, widget_id: i32);

    /// Called when a scroll event is called within the bounds of the widget.  Includes the widget ID.
    fn mouse_scrolled(&mut self, widget_id: i32, point: Point);

    // Draw routines

    /// Draws the contents of the widget, provided a `piston2d` `Context` and `GlGraphics` object.
    ///
    /// It is **highly recommended** that you call `clear_invalidate()` after the draw completes,
    /// otherwise, this will continue to be redrawn continuously (unless this is the desired
    /// behavior.)
    fn draw(&mut self, c: Context, g: &mut G2d) {
        let origin: Point = self.get_origin();
        let size: crate::core::point::Size = self.get_size();

        rectangle(
            self.get_color(),
            [
                origin.x as f64,
                origin.y as f64,
                size.w as f64,
                size.h as f64,
            ],
            c.transform,
            g,
        );

        self.clear_invalidate();
    }
}

/// This is the `BaseWidget`, which contains a top-level widget for display.  It does
/// not contain any special logic other than being a base for a display layer.
pub struct BaseWidget {
    config: Configurable,
}

/// Implementation of the constructor for the `PushrodBaseWidget`.  Creates a new base widget
/// that can be positioned anywhere on the screen.
impl BaseWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
        }
    }
}

/// Implementation of the `BaseWidget` object with the `Widget` traits implemented.
/// This function only implements `get_config`, and samples of `mouse_entered`, `mouse_exited`,
/// and `mouse_scrolled`, which currently trigger messages to the screen.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::window::*;
/// # use pushrod::widget::widget::*;
/// # fn main() {
/// #   let opengl = OpenGL::V3_2;
/// #   let mut pushrod_window: PushrodWindow = PushrodWindow::new(
/// #       WindowSettings::new("Pushrod Window", [640, 480])
/// #           .opengl(opengl)
/// #           .build()
/// #           .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)),
/// #   );
/// #
///    let mut base_widget = BaseWidget::new();
///
///    base_widget.set_origin(100, 100);
///    base_widget.set_size(200, 200);
///    base_widget.set_color([0.5, 0.5, 0.5, 1.0]);
///
///    // Widgets must be boxed, as they are trait objects.
///    let widget_id = pushrod_window.add_widget(Box::new(base_widget));
///
///    eprintln!("Added widget: ID={}", widget_id);
///
///    let mut base_widget_2 = BaseWidget::new();
///
///    base_widget_2.set_origin(125, 125);
///    base_widget_2.set_size(100, 100);
///    base_widget_2.set_color([0.75, 0.75, 0.75, 1.0]);
///
///    // Add the second widget to the top level base widget.
///    let widget_id_2 = pushrod_window.add_widget_to_parent(Box::new(base_widget_2), widget_id);
/// # }
/// ```
impl Widget for BaseWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn mouse_entered(&mut self, widget_id: i32) {
        eprintln!("[Base] Mouse entered: id={}", widget_id);
    }

    fn mouse_exited(&mut self, widget_id: i32) {
        eprintln!("[Base] Mouse exited: id={}", widget_id);
    }

    fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {
        eprintln!(
            "[Base] Mouse scrolled: x={} y={}: id={}",
            point.x, point.y, widget_id
        );
    }
}
