use enigo::{Button, Coordinate, Direction, Enigo, Key, Mouse};
use flow_model::Rect;
use std::{sync::MutexGuard, time::Duration};
use tokio::time;
use tower::{common::bincode_decode, enigo::TowerEnigo};
use tracing::debug;
use xcap::{Monitor, Window};

///休眠dur时长
pub fn sync_sleep(dur: Duration) {
    tauri::async_runtime::block_on(async move {
        time::sleep(dur).await;
    });
}

///用于keycombi相关
pub fn into_key(str: &str) -> Key {
    bincode_decode(str)
}

///获取monitor位置
pub fn get_monitor_rect_by_pid(pid: u32) -> Rect {
    get_window(pid).map(|f| get_monitor_rect(&f)).unwrap()
}
///获取应用位置
pub fn get_window_rect_by_pid(pid: u32) -> Rect {
    get_window(pid).map(|f| get_window_rect(&f)).unwrap()
}
///获取monitor位置
pub fn get_monitor_rect(window: &Window) -> Rect {
    let monitor = window.current_monitor().unwrap();
    Rect {
        x: monitor.x().unwrap(),
        y: monitor.y().unwrap(),
        width: monitor.width().unwrap(),
        height: monitor.height().unwrap(),
    }
}
///获取应用位置
pub fn get_window_rect(window: &Window) -> Rect {
    Rect {
        x: window.x().unwrap(),
        y: window.y().unwrap(),
        width: window.width().unwrap(),
        height: window.height().unwrap(),
    }
}
///获取应用位置
pub fn get_window_xy(window: &Window) -> (i32, i32) {
    (window.x().unwrap(), window.y().unwrap())
}
///获取应用位置
pub fn get_window_xy_by_pid(pid: u32) -> (i32, i32) {
    get_window(pid).map(|f| get_window_xy(&f)).unwrap()
}
///根据pid获取window对象
pub fn get_window(pid: u32) -> Option<Window> {
    let windows = Window::all();
    windows
        .as_ref()
        .map(|wins| wins.iter().find(|w_pid| w_pid.pid().unwrap() == pid))
        .ok()
        .flatten()
        .cloned()
}
///将定义的坐标转换为实际坐标
pub fn transfer_xy(
    window: &Window,
    (x, y): (i32, i32),
    width: u32,
    height: u32,
    coordinate: Coordinate,
) -> (i32, i32) {
    let Rect {
        x: win_x,
        y: win_y,
        width: win_width,
        height: win_height,
    } = get_window_rect(window);
    debug!(
        "win_pos:({win_x},{win_y})，app_dim：({width},{height})，win_dim：({win_width},{win_height})"
    );
    debug!("before transfer:{:?}", (x, y));
    let fact_x = win_width * (x as u32) / width;
    let fact_y = win_height * (y as u32) / height;

    debug!(" after transfer:({fact_x},{fact_y})");
    match coordinate {
        Coordinate::Abs => (fact_x as i32 + win_x, fact_y as i32 + win_y),
        Coordinate::Rel => (fact_x as i32, fact_y as i32),
    }
}
///将定义的rect转换为实际rect
pub fn transfer_rect(
    window: &Window,
    rect: &Rect,
    width: u32,
    height: u32,
    coordinate: Coordinate,
) -> Rect {
    let (fact_x, fact_y) = transfer_xy(window, (rect.x, rect.y), width, height, coordinate);
    let (fact_width, fact_height) = transfer_xy(
        window,
        (rect.width as i32, rect.height as i32),
        width,
        height,
        Coordinate::Rel,
    );
    Rect::new(fact_x, fact_y, fact_width as u32, fact_height as u32)
}

pub fn drag(enigo: &mut MutexGuard<TowerEnigo>, start: (i32, i32), end: (i32, i32)) {
    const DUR: Duration = Duration::from_millis(50);
    enigo.move_mouse(start.0, start.1, Coordinate::Abs).unwrap();
    sync_sleep(DUR);

    enigo.button(Button::Left, Direction::Press).unwrap();
    sync_sleep(DUR);

    enigo.move_mouse(end.0, end.1, Coordinate::Abs).unwrap();
    sync_sleep(DUR);

    enigo.button(Button::Left, Direction::Release).unwrap();
    sync_sleep(DUR);
}
#[cfg(test)]
mod tests {

    use tower::common::bincode_encode;
    use tracing::info;

    use crate::tests::init;

    use super::*;

    #[test]
    fn test_key() {
        println!("{}", bincode_encode(&Key::Escape));
    }

    #[test]
    fn test_rect() {
        init();

        let pid = 17508;
        info!("window:{:?}", get_window_rect_by_pid(pid));
        info!("monitor:{:?}", get_monitor_rect_by_pid(pid));
        info!(
            "dim:{:?}",
            get_window(pid)
                .unwrap()
                .current_monitor()
                .unwrap()
                .capture_image()
                .unwrap()
                .dimensions()
        )
    }

    #[test]
    fn test_rect1() {
        init();

        let pid = 5060;

        let window = get_window(pid).unwrap();
        let scale = window
            .current_monitor()
            .map_or(1., |m| m.scale_factor().unwrap_or(1.));
        info!("window:{:?}", get_window_rect_by_pid(pid));
    }

    #[test]
    fn test_transfer_xy() {
        init();

        let pid = 5060;

        let window = get_window(pid).unwrap();
        info!("window:{:?}", get_window_rect_by_pid(pid));
        info!(
            "{:?}",
            transfer_xy(&window, (810, 531), 1008, 756, Coordinate::Abs)
        );
    }
}
