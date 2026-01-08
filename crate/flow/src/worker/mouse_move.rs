use crate::FlowExecutor;
use enigo::{Coordinate, Mouse};

pub struct MouseMoveWorker;

impl MouseMoveWorker {
    pub fn do_work(executor: &FlowExecutor, point: (i32, i32)) -> bool {
        executor.do_with_enigo(|mut enigo| {
            let (x, y) = executor.transfer_xy_with_offset(point, Coordinate::Abs);

            enigo
                .move_mouse(x, y, Coordinate::Abs)
                .expect(format!("expect move mouse to ({},{})", x, y).as_str());
            true
        })
    }
}
