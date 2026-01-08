use std::{collections::VecDeque, rc::Rc};

use tao::{
    dpi::PhysicalPosition,
    event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
    platform::run_return::EventLoopExtRunReturn,
    window::{CursorIcon, Fullscreen, WindowBuilder},
};
use tracing::debug;

use crate::{to_point, Rectangle};

///(rect,monitor_pos)
pub fn get_guaguale_rect() -> Option<(Rectangle, (i32, i32))> {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("第五塔灵-刮刮乐")
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
        .with_decorations(true)
        .with_transparent(true)
        .build(&event_loop)
        .unwrap();
    window.set_cursor_icon(CursorIcon::Crosshair);

    let mut res = {
        let window = Rc::new(window);
        let record = GuaRecord::new();
        let quit = false;
        (window, record, quit)
    };

    while !res.2 {
        let (ref window, ref mut record, ref mut quit) = res;
        event_loop.run_return(move |event, target, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        if Key::Escape == logical_key {
                            *control_flow = ControlFlow::Exit;
                            *quit = true;
                        }

                        //取消,重新设置
                        if Key::Character("z") == logical_key {
                            record.clear();
                        }
                        //保存
                        if Key::Character("d") == logical_key {
                            if record.save() {
                                *control_flow = ControlFlow::Exit;
                                *quit = true;
                            }
                        }
                    }
                    WindowEvent::MouseInput {
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        ..
                    } => {
                        if let Ok(pos) = target.cursor_position() {
                            record.push(pos);
                        } else {
                            *control_flow = ControlFlow::Exit;
                            *quit = true;
                        }
                    }
                    _ => (),
                },
                _ => {}
            }
        });
    }
    let monitor = event_loop
        .cursor_position()
        .map(|cursor| event_loop.monitor_from_point(cursor.x, cursor.y))
        .unwrap();
    let monitor_pos = monitor.map_or((0, 0), |m| (m.position().x, m.position().y));

    res.1.get_store().map(|rect| {
        debug!("rect:{:?}, monitor_pos:{:?}", rect, monitor_pos);
        (rect, monitor_pos)
    })
}

struct GuaRecord {
    queue: VecDeque<PhysicalPosition<f64>>,
    store: Vec<Rectangle>,
}

impl GuaRecord {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            store: Vec::new(),
        }
    }
    fn push(&mut self, value: PhysicalPosition<f64>) {
        if self.queue.len() == 2 {
            self.queue.pop_back();
        }
        self.queue.push_back(value);
    }

    fn pop(&mut self) {
        self.queue.pop_back();
    }

    fn clear(&mut self) {
        self.queue.clear();
    }

    ///保存当前操作，结束截图返回true
    fn save(&mut self) -> bool {
        if self.queue.len() == 2 {
            let rect = Rectangle::new(
                to_point(self.queue.pop_front().unwrap()),
                to_point(self.queue.pop_front().unwrap()),
            );
            self.store.push(rect);
        }
        self.store.len() == 1
    }
    fn get_store(&self) -> Option<Rectangle> {
        if self.store.len() < 1 {
            None
        } else {
            Some(self.store.get(0).unwrap().clone())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image() {
        // get_rects();
    }
}
