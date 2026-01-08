use std::{default, rc::Rc};

use tao::{
    event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::Key,
    platform::run_return::EventLoopExtRunReturn,
    window::{CursorIcon, Fullscreen, WindowBuilder},
};
use tracing::debug;

use crate::{RecordQueue, Rectangle};

///(match_rect,match_template,monitor_pos)
pub fn get_image_rects() -> Option<(Rectangle, Rectangle, (i32, i32))> {
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("第五塔灵-截图")
        .with_decorations(true)
        .with_transparent(true);

    let monitor = event_loop
        .cursor_position()
        .map(|cursor| event_loop.monitor_from_point(cursor.x, cursor.y))
        .unwrap();
    debug!(
        "current monitor:{:?}",
        monitor.as_ref().map_or(None, |f| f.name())
    );
    let window = window
        .with_fullscreen(Some(Fullscreen::Borderless(monitor.clone())))
        .build(&event_loop)
        .unwrap();

    window.set_cursor_icon(CursorIcon::Crosshair);

    let mut res = {
        let window = Rc::new(window);
        let record = RecordQueue::new();
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
    let monitor_pos = monitor.map_or((0, 0), |m| (m.position().x, m.position().y));
    res.1.get_store().map(|(match_rect, match_template)| {
        debug!(
            "match_rect:{:?}, match_template:{:?}, monitor_pos:{:?}",
            match_rect, match_template, monitor_pos
        );
        (match_rect, match_template, monitor_pos)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_image() {
        // get_rects();
    }
}
