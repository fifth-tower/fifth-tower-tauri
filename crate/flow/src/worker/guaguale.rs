use enigo::Coordinate;
use flow_model::Rect;
use rand::Rng;

use crate::{drag, FlowExecutor};

pub(crate) struct GuagualeWorker;

impl GuagualeWorker {
    pub fn do_work(executor: &FlowExecutor, rect: &Rect) -> bool {
        executor.do_with_enigo(|mut enigo| {
            let mut rng = rand::rng();
            let var = rng.random_range(0..2);
            let (cursor_size, _) = executor.transfer_xy((16, 16), Coordinate::Rel);

            let rect = executor.transfer_rect(rect, Coordinate::Abs);
            let mut sides = if var == 0 {
                forward_x(&rect, cursor_size)
            } else {
                forward_y(&rect, cursor_size)
            };

            let var = rng.random_range(0..2);
            if var == 0 {
                sides.reverse();
            }

            let mut switch = false;
            for (mut start, mut end) in sides {
                if switch {
                    std::mem::swap(&mut start, &mut end);
                    switch = !switch;
                }
                drag(&mut enigo, start, end);
            }
            true
        })
    }
}

///向下/上垂直刮
fn forward_x(rect: &Rect, cursor_size: i32) -> Vec<((i32, i32), (i32, i32))> {
    let mut ret = vec![];
    let mut x = 0;
    loop {
        if x > rect.width as i32 {
            x = rect.width as i32;
        }
        let start = (rect.x + x, rect.y);
        let end = (rect.x + x, rect.y + (rect.height as i32));

        ret.push((start, end));

        if x >= rect.width as i32 {
            break;
        }
        x += cursor_size;
    }
    return ret;
}
///向左/右水平刮
fn forward_y(rect: &Rect, cursor_size: i32) -> Vec<((i32, i32), (i32, i32))> {
    let mut ret = vec![];
    let mut y = 0;
    loop {
        if y > rect.height as i32 {
            y = rect.height as i32;
        }
        let start = (rect.x, rect.y + y);
        let end = (rect.x + (rect.width as i32), rect.y + y);

        ret.push((start, end));

        if y >= rect.height as i32 {
            break;
        }
        y += cursor_size;
    }
    return ret;
}
