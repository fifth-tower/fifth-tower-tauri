use std::{ops::Neg, rc::Rc};

use tao::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
    platform::run_return::EventLoopExtRunReturn,
    window::{CursorIcon, WindowBuilder},
};
use tracing::info;

pub fn get_scroll() -> Option<(i32, i32)> {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("第五塔灵-记录滚动前，需要先选中我噢")
        .with_decorations(true)
        .with_transparent(true)
        .with_always_on_top(true)
        .with_inner_size(LogicalSize::new(500, 0))
        .build(&event_loop)
        .unwrap();
    window.set_cursor_icon(CursorIcon::Crosshair);

    let mut res = {
        let window = Rc::new(window);
        let scroll = ScrollData::new();
        let quit = false;
        (window, scroll, quit)
    };

    while !res.2 {
        let (ref window, ref mut scroll, ref mut quit) = res;
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
                            scroll.exit();
                            *control_flow = ControlFlow::Exit;
                            *quit = true;
                        }

                        //取消,重新设置
                        if Key::Character("z") == logical_key {
                            scroll.reset();
                        }
                        //保存
                        if Key::Character("d") == logical_key {
                            *control_flow = ControlFlow::Exit;
                            *quit = true;
                        }
                    }
                    _ => (),
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseWheel { delta, .. } => match delta {
                        tao::event::MouseScrollDelta::LineDelta(x, y) => {
                            scroll.add(x, y);
                        }
                        tao::event::MouseScrollDelta::PixelDelta(p) => {
                            info!("mouse wheel Pixel Delta: ({},{})", p.x, p.y);
                        }
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
    res.1.to_data()
}

struct ScrollData(Option<(f32, f32)>);
impl ScrollData {
    fn new() -> Self {
        Self(Some((0.0, 0.0)))
    }
    fn add(&mut self, x: f32, y: f32) {
        if let Some((sx, sy)) = self.0.as_mut() {
            *sx += x;
            *sy += y;
        }
    }
    fn to_data(&self) -> Option<(i32, i32)> {
        self.0.map(|(x, y)| (x.neg() as i32, y.neg() as i32))
    }
    fn reset(&mut self) {
        if let Some((sx, sy)) = self.0.as_mut() {
            *sx = 0.0;
            *sy = 0.0;
        }
    }
    fn exit(&mut self) {
        self.0 = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut data = ScrollData::new();
        data.add(1.0, 1.0);
        println!("ScrollData: {:?}", data.to_data());
    }
}
