// Pushrod Widget Library
// Slider Widget
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
use crate::render::{Points, Size, POINT_X, POINT_Y, SIZE_HEIGHT, SIZE_WIDTH};

use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use crate::render::canvas_helper::CanvasHelper;
use crate::render::layout_cache::LayoutContainer;
use crate::render::texture_cache::TextureCache;
use crate::render::texture_store::TextureStore;
use crate::widgets::slider_widget::SliderOrientation::{SliderHorizontal, SliderVertical};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::any::Any;
use std::collections::HashMap;

/// This is the callback type that is used when an `on_value_changed` callback is triggered from this
/// `Widget`.
pub type OnValueChangedCallbackType =
    Option<Box<dyn FnMut(&mut SliderWidget, &[WidgetContainer], &[LayoutContainer], u32)>>;

/// These are the possible slider orientations.
#[derive(PartialEq, Clone)]
pub enum SliderOrientation {
    /// Indicates a horizontally controllable slider.
    SliderHorizontal,

    /// Indicates a vertically controllable slider.
    SliderVertical,
}

/// This is the storage object for the `SliderWidget`.  It stores the config, properties, callback registry.
pub struct SliderWidget {
    config: WidgetConfig,
    system_properties: HashMap<i32, String>,
    callback_registry: CallbackRegistry,
    texture_store: TextureStore,
    min: u32,
    max: u32,
    current: u32,
    orientation: SliderOrientation,
    in_bounds: bool,
    active: bool,
    originated: bool,
    on_value_changed: OnValueChangedCallbackType,
}

/// This is the implementation of the `SliderWidget`, a control that draws a bounds line indicator, and a
/// draggable slider.
impl SliderWidget {
    /// Creates a new `SliderWidget` given the `x, y, w, h` coordinates, sets the `min` and `max` values,
    /// the `current` value, and the `orientation` of the slider as drawn.
    pub fn new(
        points: Points,
        size: Size,
        min: u32,
        max: u32,
        current: u32,
        orientation: SliderOrientation,
    ) -> Self {
        Self {
            config: WidgetConfig::new(points, size),
            system_properties: HashMap::new(),
            callback_registry: CallbackRegistry::new(),
            texture_store: TextureStore::default(),
            min,
            max,
            current,
            orientation,
            in_bounds: false,
            active: false,
            originated: false,
            on_value_changed: None,
        }
    }

    /// Assigns the callback closure that will be used when the `Widget` changes value.
    pub fn on_value_changed<F>(&mut self, callback: F)
    where
        F: FnMut(&mut SliderWidget, &[WidgetContainer], &[LayoutContainer], u32) + 'static,
    {
        self.on_value_changed = Some(Box::new(callback));
    }

    /// Internal function that triggers the `on_value_changed` callback.
    fn call_value_changed_callback(
        &mut self,
        widgets: &[WidgetContainer],
        layouts: &[LayoutContainer],
    ) {
        if let Some(mut cb) = self.on_value_changed.take() {
            cb(self, widgets, layouts, self.current);
            self.on_value_changed = Some(cb);
        }
    }
}

impl CanvasHelper for SliderWidget {}

/// This is the `Widget` implementation of the `SliderWidget`.
impl Widget for SliderWidget {
    /// Draws the `SliderWidget` contents.
    fn draw(&mut self, c: &mut Canvas<Window>, _t: &mut TextureCache) -> Option<&Texture> {
        if self.get_config().invalidated() {
            let bounds = self.get_config().get_size(CONFIG_SIZE);

            self.texture_store
                .create_or_resize_texture(c, bounds[0] as u32, bounds[1] as u32);

            // Draw base - three lines in the center
            let half_height = (self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT] / 2) as i32;
            let half_width = (self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH] / 2) as i32;
            let height = (self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT]) as i32;
            let width = (self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH]) as i32;
            let base_color = self.get_color(CONFIG_COLOR_BASE);
            let orientation = self.orientation.clone();
            let min = self.min;
            let max = self.max;
            let current = self.current;

            c.with_texture_canvas(self.texture_store.get_mut_ref(), |texture| {
                texture.set_draw_color(base_color);
                texture.clear();

                if orientation == SliderHorizontal {
                    texture.set_draw_color(Color::RGB(192, 192, 192));
                    texture
                        .draw_line(
                            Point::new(10, half_height),
                            Point::new(width - 10, half_height),
                        )
                        .unwrap();

                    texture
                        .draw_line(
                            Point::new(10, half_height - 1),
                            Point::new(width - 10, half_height - 1),
                        )
                        .unwrap();

                    texture
                        .draw_line(
                            Point::new(10, half_height + 1),
                            Point::new(width - 10, half_height + 1),
                        )
                        .unwrap();

                    // Draw slider at current value
                    let full_range = max - min;
                    let slider_center =
                        ((width as f64 / full_range as f64) * (current - min) as f64) as u32;
                    let slider_start = if slider_center >= width as u32 - 15 {
                        width as u32 - 30
                    } else if slider_center <= 15 {
                        0
                    } else {
                        slider_center - 15
                    };

                    texture.set_draw_color(base_color);
                    texture
                        .fill_rect(Rect::new(slider_start as i32, 0, 30, bounds[SIZE_HEIGHT]))
                        .unwrap();

                    texture.set_draw_color(Color::RGB(0, 0, 0));
                    texture
                        .draw_rect(Rect::new(slider_start as i32, 0, 30, bounds[SIZE_HEIGHT]))
                        .unwrap();
                } else if orientation == SliderVertical {
                    // Draw base - three lines in the center

                    texture.set_draw_color(Color::RGB(192, 192, 192));
                    texture
                        .draw_line(
                            Point::new(half_width, 10),
                            Point::new(half_width, height - 10),
                        )
                        .unwrap();

                    texture
                        .draw_line(
                            Point::new(half_width - 1, 10),
                            Point::new(half_width - 1, height - 10),
                        )
                        .unwrap();

                    texture
                        .draw_line(
                            Point::new(half_width + 1, 10),
                            Point::new(half_width + 1, height - 10),
                        )
                        .unwrap();

                    // Draw slider at current value
                    let full_range = max - min;
                    let slider_center =
                        ((height as f64 / full_range as f64) * (current - min) as f64) as u32;
                    let slider_start = if slider_center >= height as u32 - 15 {
                        height as u32 - 30
                    } else if slider_center <= 15 {
                        0
                    } else {
                        slider_center - 15
                    };

                    texture.set_draw_color(base_color);
                    texture
                        .fill_rect(Rect::new(0, slider_start as i32, bounds[SIZE_WIDTH], 30))
                        .unwrap();

                    texture.set_draw_color(Color::RGB(0, 0, 0));
                    texture
                        .draw_rect(Rect::new(0, slider_start as i32, bounds[SIZE_WIDTH], 30))
                        .unwrap();
                }
            })
            .unwrap();
        }

        self.texture_store.get_optional_ref()
    }

    /// When a mouse enters the bounds of the `Widget`, this function is triggered.
    fn mouse_entered(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = true;
    }

    /// When a mouse exits the bounds of the `Widget`, this function is triggered.
    fn mouse_exited(&mut self, _widgets: &[WidgetContainer], _layouts: &[LayoutContainer]) {
        self.in_bounds = false;
    }

    /// When a mouse is moved in the bounds of this `Widget`, this function is triggered.
    fn mouse_moved(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        points: Points,
    ) {
        if self.in_bounds && self.active && self.originated {
            if self.orientation == SliderHorizontal {
                let width = (self.get_config().get_size(CONFIG_SIZE)[SIZE_WIDTH]) as i32;
                let position_x =
                    points[POINT_X] - self.get_config().get_point(CONFIG_ORIGIN)[POINT_X] as i32;
                let percentage = position_x as f64 / width as f64;
                let full_range = self.max - self.min;
                let actual = (percentage * full_range as f64) as u32;

                self.current = self.min + actual;

                self.get_config().set_invalidated(true);
                self.call_value_changed_callback(_widgets, _layouts);
            } else if self.orientation == SliderVertical {
                let height = (self.get_config().get_size(CONFIG_SIZE)[SIZE_HEIGHT]) as i32;
                let position_y =
                    points[POINT_Y] - self.get_config().get_point(CONFIG_ORIGIN)[POINT_Y] as i32;
                let percentage = position_y as f64 / height as f64;
                let full_range = self.max - self.min;
                let actual = (percentage * full_range as f64) as u32;

                self.current = self.min + actual;

                self.get_config().set_invalidated(true);
                self.call_value_changed_callback(_widgets, _layouts);
            }
        }
    }

    /// Handles the scrolling functionality.
    fn mouse_scrolled(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        points: Points,
    ) {
        let mut current_i32 = self.current as i32;

        if self.orientation == SliderHorizontal {
            current_i32 += points[POINT_X];
        } else if self.orientation == SliderVertical {
            current_i32 += -points[POINT_Y];
        }

        if current_i32 >= self.max as i32 {
            current_i32 = self.max as i32;
        } else if current_i32 <= self.min as i32 {
            current_i32 = self.min as i32;
        }

        self.current = current_i32 as u32;

        self.get_config().set_invalidated(true);
        self.call_value_changed_callback(_widgets, _layouts);
    }

    /// Overrides the `button_clicked` callback to handle toggling.
    fn button_clicked(
        &mut self,
        _widgets: &[WidgetContainer],
        _layouts: &[LayoutContainer],
        _button: u8,
        _clicks: u8,
        _state: bool,
    ) {
        if _button == 1 {
            if _state {
                self.active = true;
                self.originated = true;
            } else {
                self.active = false;
                self.originated = false;
            }

            self.get_config().set_invalidated(true);
        }

        self.button_clicked_callback(_widgets, _layouts, _button, _clicks, _state);
    }

    default_widget_functions!();
    default_widget_properties!();
    default_widget_callbacks!();
}
