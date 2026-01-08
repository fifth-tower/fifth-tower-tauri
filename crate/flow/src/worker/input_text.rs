use std::time::Duration;

use crate::{sync_sleep, FlowExecutor};
use enigo::{Coordinate, Keyboard, Mouse};

pub(crate) struct InputTextWorker;

impl InputTextWorker {
    pub fn do_work(executor: &FlowExecutor, point: (i32, i32), text: &str) -> bool {
        executor.do_with_enigo(|mut enigo| {
            let (x, y) = executor.transfer_xy_with_offset(point, Coordinate::Abs);

            enigo
                .move_mouse(x, y, Coordinate::Abs)
                .expect(format!("expect move mouse to ({},{})", x, y).as_str());

            sync_sleep(Duration::from_millis(50));

            enigo.text(text).unwrap();
            true
        })
    }
}
