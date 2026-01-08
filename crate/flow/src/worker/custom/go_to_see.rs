use crate::FlowExecutor;
use enigo::{Coordinate, Mouse};
use tower::enigo::TowerEnigo;

pub struct GoToSeeWorker;

impl GoToSeeWorker {
    pub fn do_work(executor: &FlowExecutor, point: (i32, i32)) -> bool {
        executor.do_with_enigo(|mut _enigo| {
            let mut enigo = TowerEnigo::new_frontend();
            let (x, y) = executor.transfer_xy(point, Coordinate::Abs);

            enigo
                .move_mouse(x, y, Coordinate::Abs)
                .expect(format!("expect move mouse to ({},{})", x, y).as_str());
            true
        })
    }
}
