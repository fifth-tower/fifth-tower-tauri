use crate::FlowExecutor;
use enigo::{Button, Coordinate, Direction, Mouse};

pub(crate) struct MouseClickWorker;

impl MouseClickWorker {
    pub fn do_work(executor: &FlowExecutor, point: (i32, i32)) -> bool {
        executor.do_with_enigo(|mut enigo| {
            let (x, y) = executor.transfer_xy_with_offset(point, Coordinate::Abs);

            enigo
                .move_mouse(x, y, Coordinate::Abs)
                .expect(format!("expect move mouse to ({},{})", x, y).as_str());
            enigo
                .button(Button::Left, Direction::Click)
                .expect(format!("expect click mouse at ({},{})", x, y).as_str());
            true
        })
    }
}
