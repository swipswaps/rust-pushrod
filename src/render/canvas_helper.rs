// Pushrod Rendering Library
// Canvas Helper Trait
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

use crate::render::widget::Widget;
use crate::render::widget_config::{CONFIG_BORDER_WIDTH, CONFIG_SIZE};
use crate::render::{SIZE_HEIGHT, SIZE_WIDTH};
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

/// This trait is used in conjunction with `Widget`s or anything else that draws to a `Canvas` object.
/// It provides convenience methods to provide drawing functions common to `Widget`s.  All points and
/// dimensions are relative to the position of the `Widget`, so no translation is necessary.
///
/// To implement this trait in your `Widget`, all you have to do is:
/// ```ignore
/// impl CanvasHelper for (myWidget) { }
/// ```
pub trait CanvasHelper: Widget {
    /// Draws a point in the `Canvas`.
    fn draw_point(&mut self, c: &mut Canvas<Window>, x: i32, y: i32) {
        let point = Point::new(self.get_config().to_x(x), self.get_config().to_y(y));

        c.draw_point(point).unwrap();
    }

    /// Draws a box around the bounding area of the `Widget`.
    fn draw_bounding_box(&mut self, c: &mut Canvas<Window>) {
        let border = self.get_config().get_numeric(CONFIG_BORDER_WIDTH);

        for i in 0..border {
            c.draw_rect(Rect::new(
                self.get_config().to_x(i as i32),
                self.get_config().to_y(i as i32),
                self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH] - (i * 2) as u32,
                self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT] - (i * 2) as u32,
            ))
            .unwrap();
        }
    }

    /// Returns a `Rect` destination object
    fn get_rect_dest(&mut self) -> Rect {
        Rect::new(
            self.get_config().to_x(0),
            self.get_config().to_y(0),
            self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH],
            self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT],
        )
    }
}
