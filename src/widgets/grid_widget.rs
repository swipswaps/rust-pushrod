// Pushrod Widget Library
// Grid Widget
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

use crate::render::callbacks::CallbackRegistry;
use crate::render::widget::*;
use crate::render::widget_cache::WidgetContainer;
use crate::render::widget_config::*;
use crate::render::{Points, Size, SIZE_HEIGHT, SIZE_WIDTH};

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::render::canvas_helper::CanvasHelper;
use crate::render::layout_cache::LayoutContainer;
use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::any::Any;
use std::collections::HashMap;

/// This is the storage object for the `GridWidget`.  It stores the config, properties, callback registry.
pub struct GridWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    texture_store: TextureStore,
    grid_size: u32,
    grid_connections: bool,
}

impl CanvasHelper for GridWidget {}

/// This is the implementation of the `GridWidget`, a control that displays a grid inside its bounds.
impl GridWidget {
    /// Creates a new `GridWidget` given the `x, y, w, h` coordinates, sets the grid size.
    pub fn new(points: Points, size: Size, grid_size: u32, grid_connections: bool) -> Self {
        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::default(),
            grid_size,
            grid_connections,
        }
    }

    /// Adjusts the size of the grid, redrawing the object.
    pub fn set_grid_size(&mut self, grid_size: u32) {
        self.grid_size = grid_size;
        self.get_config().set_invalidated(true);
    }

    /// Turns on or off the grid, showing a grid or dots.
    pub fn set_grid_connections(&mut self, grid_connections: bool) {
        self.grid_connections = grid_connections;
        self.get_config().set_invalidated(true);
    }
}

/// This is the `Widget` implementation of the `GridWidget`.
impl Widget for GridWidget {
    /// Draws the `GridWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>, _t: &mut TextureCache) -> Option<&Texture> {
        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            let base_color = self.get_color(CONFIG_COLOR_BASE);
            let border_color = self.get_config().get_color(CONFIG_COLOR_BORDER);
            let size = self.get_config().get_size(CONFIG_SIZE);
            let grid_connections = self.grid_connections;
            let grid_size = self.grid_size as usize;

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                if grid_connections {
                    texture.set_draw_color(Color::RGB(192, 192, 192));

                    for i in (0..size[SIZE_WIDTH]).step_by(grid_size) {
                        texture
                            .draw_line(
                                Point::new(i as i32, 0),
                                Point::new(i as i32, size[SIZE_HEIGHT] as i32),
                            )
                            .unwrap();
                    }

                    for i in (0..size[SIZE_HEIGHT]).step_by(grid_size) {
                        texture
                            .draw_line(
                                Point::new(0, i as i32),
                                Point::new(size[SIZE_WIDTH] as i32, i as i32),
                            )
                            .unwrap();
                    }
                } else {
                    texture.set_draw_color(Color::RGB(0, 0, 0));

                    for x in (0..size[SIZE_WIDTH]).step_by(grid_size) {
                        for y in (0..size[SIZE_HEIGHT]).step_by(grid_size) {
                            texture.draw_point(Point::new(x as i32, y as i32)).unwrap();
                        }
                    }
                }

                texture.set_draw_color(border_color);
                texture
                    .draw_rect(Rect::new(0, 0, size[0], size[1]))
                    .unwrap();
            })
            .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
