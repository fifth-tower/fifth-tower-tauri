use crate::{sync_sleep, FlowExecutor};
use enigo::{Axis, Coordinate, Mouse};
use flow_model::ScrollAttr;
use std::{sync::MutexGuard, time::Duration};
use tower::enigo::TowerEnigo;

pub(crate) struct MouseScrollWorker;

impl MouseScrollWorker {
    pub fn do_work(executor: &FlowExecutor, attr: &ScrollAttr) -> bool {
        executor.do_with_enigo(|mut enigo| {
            let &ScrollAttr {
                x,
                y,
                len,
                is_vertical,
                ..
            } = attr;

            let (x, y) = executor.transfer_xy_with_offset((x, y), Coordinate::Abs);

            enigo
                .move_mouse(x, y, Coordinate::Abs)
                .expect(format!("expect move mouse to ({},{})", x, y).as_str());
            sync_sleep(Duration::from_millis(10));

            if executor.is_virtual_scroll() {
                mouse_scroll_virtual(enigo, len, is_vertical);
            } else {
                mouse_scroll(enigo, len, is_vertical);
            }
            true
        })
    }
}

fn mouse_scroll_virtual(mut enigo: MutexGuard<TowerEnigo>, len: i32, is_vertical: bool) {
    let mut i = 0;
    while i < len.abs() {
        enigo
            .scroll(
                len / len.abs(),
                if is_vertical {
                    Axis::Vertical
                } else {
                    Axis::Horizontal
                },
            )
            .unwrap();

        sync_sleep(Duration::from_millis(100));

        i += 1;
    }
}

fn mouse_scroll(mut enigo: MutexGuard<TowerEnigo>, len: i32, is_vertical: bool) {
    enigo
        .scroll(
            len,
            if is_vertical {
                Axis::Vertical
            } else {
                Axis::Horizontal
            },
        )
        .unwrap();
}
