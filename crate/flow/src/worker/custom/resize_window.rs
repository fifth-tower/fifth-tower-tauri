use flow_model::Rect;
use tracing::debug;

use crate::{drag, get_monitor_rect, get_window_rect, is_same_size, FlowExecutor};

pub struct ResizeWindowWorker;
impl ResizeWindowWorker {
    pub fn do_work(executor: &FlowExecutor) -> bool {
        let window = executor.window();
        let app_wh = (executor.app().width, executor.app().height);

        let same_width = move || is_same_size(window.width().unwrap(), app_wh.0);
        let same_height = move || is_same_size(window.height().unwrap(), app_wh.1);
        if same_width() && same_height() {
            return true;
        }
        executor.do_with_enigo(|mut enigo| {
            //move to (0,50)
            let win_width = window.width().unwrap();
            let rect = get_window_rect(window);
            debug!("drag up:{:?}", rect);
            let monitor_rect = get_monitor_rect(window);
            drag(
                &mut enigo,
                DragDirect::Up.start(&rect),
                (monitor_rect.x + win_width as i32 / 2, monitor_rect.y + 50),
            );
            if same_width() && same_height() {
                return true;
            }
            let rect = get_window_rect(window);
            debug!("drag right start:{:?}", rect);
            drag(
                &mut enigo,
                DragDirect::Right.start(&rect),
                DragDirect::Right.end(&rect, app_wh),
            );
            if same_width() && same_height() {
                return true;
            }
            let rect = get_window_rect(window);
            debug!("drag down start:{:?}", rect);
            drag(
                &mut enigo,
                DragDirect::Down.start(&rect),
                DragDirect::Down.end(&rect, app_wh),
            );

            let rect = get_window_rect(window);
            debug!("drag down end:{:?}", rect);
            same_width() && same_height()
        })
    }
}

enum DragDirect {
    Left,
    Right,
    Up,
    Down,
}
impl DragDirect {
    fn start(&self, rect: &Rect) -> (i32, i32) {
        let &Rect {
            x: win_x,
            y: win_y,
            width: win_width,
            height: win_height,
        } = rect;
        match self {
            Self::Left => (win_x - 1, win_y + win_height as i32 / 2),
            Self::Right => (win_x + win_width as i32, win_y + win_height as i32 / 2),
            Self::Up => (win_x + win_width as i32 / 2, win_y - 1),
            Self::Down => (win_x + win_width as i32 / 2, win_y + win_height as i32 + 1),
        }
    }
    fn end(&self, rect: &Rect, (width, height): (u32, u32)) -> (i32, i32) {
        let &Rect {
            x: win_x,
            y: win_y,
            width: win_width,
            height: win_height,
        } = rect;
        match self {
            Self::Left => (
                win_x + win_width as i32 - width as i32 - 1,
                win_y + win_height as i32 / 2,
            ),
            Self::Right => (win_x + width as i32, win_y + win_height as i32 / 2),
            Self::Up => (win_x + win_height as i32 / 2 - height as i32, win_y - 1),
            Self::Down => (win_x + win_width as i32 / 2, win_y + height as i32 + 1),
        }
    }
}
#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use tracing::{debug, info};

    use crate::{tests::init, *};
    #[test]
    fn test_worker() {
        init();

        let app_id = "FQAAAAAAAADmoqblubvopbmuLjvvJrml7bnqbo=";
        let pid = 5060;

        let app = record::load_app(&Dir::Record, app_id);
        let app_wh = (app.width, app.height);
        let resource = Arc::new(FLowResource::new());
        let executor = FlowExecutor::new(pid, app, resource, Dir::Record, Box::new(|_| {}));
        let ret = ResizeWindowWorker::do_work(&executor);
        debug!("app_setting size:{:?}", app_wh);
        info!("window:{:?}", get_window_rect_by_pid(pid));
        info!(ret);
    }
}
