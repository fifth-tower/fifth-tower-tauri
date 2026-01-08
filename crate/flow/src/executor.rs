use std::{
    ops::{Add, Deref, Sub},
    sync::{Arc, Mutex, MutexGuard},
    time::Duration,
};

use enigo::{Coordinate, Mouse};
use flow_model::*;
use fs_extra::dir;
use image::*;
use rand::Rng;
use tower::{common::Holder, enigo::TowerEnigo};
use tracing::debug;
use xcap::Window;

use super::*;
use crate::{sync_sleep, FLowResource};

pub struct FlowExecutor {
    resource: Arc<FLowResource>,
    window: Window,
    images: Holder<String, DynamicImage>,
    app: App,
    flow_ids: Vec<String>,
    dir: Dir,
    log_handle_fn: Box<dyn Fn(LogMessage)>,
    action_level: u32,
    pid: u32,
    cursor_pos: (i32, i32),
    is_of: bool,
    engio: Mutex<TowerEnigo>,
}
///流程处理相关
impl FlowExecutor {
    pub fn new(
        pid: u32,
        app: App,
        resource: Arc<FLowResource>,
        dir: Dir,
        log_handle_fn: Box<dyn Fn(LogMessage)>,
    ) -> Self {
        let window = get_window(pid).expect(format!("expect a window with pid: {}", pid).as_str());
        let engio = if app.backend {
            TowerEnigo::new_backend(pid)
        } else {
            TowerEnigo::new_frontend()
        };
        let cursor_pos = engio.location().unwrap();
        resource.set_running(pid);
        debug!("pid:{pid},win:{:?}", get_window_rect(&window));

        let images = Holder::<String, DynamicImage>::new(Box::new(|path: String| {
            open(path.clone()).expect(format!("Failed to open template image:{}", path).as_str())
        }));
        dir::create_all(Dir::Tmp.image(&pid.to_string(), None).0, true).unwrap();

        Self {
            pid,
            resource,
            window,
            images,
            app,
            flow_ids: vec![],
            dir,
            log_handle_fn,
            action_level: 0,
            cursor_pos,
            is_of: false,
            engio: Mutex::new(engio),
        }
    }
    pub(crate) fn is_stop(&self) -> bool {
        self.resource.is_stop(self.pid)
    }
    pub(crate) fn write_log(&mut self, content: LogContent) {
        let message = LogMessage {
            action_level: self.action_level,
            content: content.clone(),
        };
        debug!("{:?}", message);
        self.log_handle_fn.deref()(message);

        if matches!(content, LogContent::Stop) {
            self.cursor_back();
        }
    }

    pub(crate) fn add_action_level(&mut self) {
        if self.action_level == 0 {
            self.resource.set_running(self.pid);
        }
        self.action_level += 1;
    }
    pub(crate) fn sub_action_level(&mut self) {
        self.action_level -= 1;
        if self.action_level == 0 {
            self.resource.set_stopped(self.pid);
            self.cursor_back();
        }
    }
    pub fn cursor_back(&self) {
        if self.is_backend() {
            return;
        }
        self.do_with_enigo(|mut enigo| {
            enigo
                .move_mouse(self.cursor_pos.0, self.cursor_pos.1, Coordinate::Abs)
                .unwrap();
        });
    }

    pub(crate) fn push_flow_id(&mut self, flow_id: String) {
        self.flow_ids.push(flow_id);
    }
    pub(crate) fn pop_flow_id(&mut self) {
        self.flow_ids.pop();
    }
    pub(crate) fn is_minimized(&self) -> bool {
        self.window.is_minimized().unwrap()
    }
    pub(crate) fn flow(&self, flow_id: &str) -> Flow {
        self.app
            .flows
            .iter()
            .find(|fid| fid.flow_id.eq(flow_id))
            .expect(format!("expect sub flow with id: {}", flow_id).as_str())
            .clone()
    }
}
///action执行相关
impl FlowExecutor {
    pub(crate) fn do_with_enigo<F, T>(&self, f: F) -> T
    where
        F: Fn(MutexGuard<TowerEnigo>) -> T,
    {
        self.resource.do_with_enigo(
            || {
                let enigo = self.engio.lock().unwrap();
                f(enigo)
            },
            !self.is_of,
        )
    }
    pub(crate) fn start_of(&mut self) {
        self.is_of = true;
    }
    pub(crate) fn end_of(&mut self) {
        self.is_of = false;
    }
    pub(crate) fn window(&self) -> &Window {
        &self.window
    }
    pub(crate) fn app(&self) -> &App {
        &self.app
    }
    ///根据template获取文件路径
    pub(crate) fn get_template_image(&mut self, template: &str) -> DynamicImage {
        let path = {
            if let Dir::Script(_, zip_id) = &self.dir {
                format!(
                    "{}/{}/{}/{}",
                    self.dir.path(),
                    &self.app.app_id,
                    zip_id,
                    template
                )
            } else {
                format!("{}/{}", self.dir.app(&self.app.app_id), template)
            }
        };
        self.images.get(path)
    }
    ///return app.virtual_scroll
    pub fn is_virtual_scroll(&self) -> bool {
        self.app.virtual_scroll
    }
    pub fn is_backend(&self) -> bool {
        self.app.backend
    }
    pub fn get_window_rect(&self) -> Rect {
        get_window_rect(&self.window)
    }
    pub fn sync_sleep_with_offset(&self, dur: &Duration) {
        let dur = offset_sleep(&self.app, dur);
        sync_sleep(dur);
    }
    pub fn transfer_rect(&self, rect: &Rect, coordinate: Coordinate) -> Rect {
        transfer_rect(
            &self.window,
            rect,
            self.app.width,
            self.app.height,
            coordinate,
        )
    }
    ///将参考坐标转换为实际坐标
    pub fn transfer_xy(&self, point: (i32, i32), coordinate: Coordinate) -> (i32, i32) {
        transfer_xy(
            &self.window,
            point,
            self.app.width,
            self.app.height,
            coordinate,
        )
    }
    ///将参考坐标转换为实际坐标
    pub fn transfer_xy_with_offset(&self, point: (i32, i32), coordinate: Coordinate) -> (i32, i32) {
        let point = self.offset_cursor(point);

        self.transfer_xy(point, coordinate)
    }
    ///获取随机偏移后的坐标
    pub fn offset_cursor(&self, cursor: (i32, i32)) -> (i32, i32) {
        let curor_offset = self.app.cursor_offset;
        if curor_offset.0 < 0 || curor_offset.1 < 0 {
            panic!("app.cursor_offset参数不能为负数")
        }
        let mut rng = rand::rng();
        let x = rng.random_range(0 - curor_offset.0..curor_offset.0);
        let y = rng.random_range((0 - curor_offset.1) as i32..curor_offset.1 as i32);

        (0.max(cursor.0 + x), 0.max(cursor.1 + y))
    }
}
///获取随机偏移后的时长
pub fn offset_sleep(app: &App, dur: &Duration) -> Duration {
    if app.cursor_offset.0 < 0 || app.cursor_offset.1 < 0 {
        panic!("app.offset_sleep参数不能为负数")
    }
    //若dur小于sleep_offset*10，则不偏移
    if dur.as_millis() < ((app.sleep_offset * 10) as u128) {
        return dur.clone();
    }

    let mut rng = rand::rng();
    let millis = rng.random_range(0 - app.sleep_offset..app.sleep_offset);
    if millis.is_negative() {
        dur.sub(Duration::from_millis(millis.abs() as u64))
    } else {
        dur.add(Duration::from_millis(millis as u64))
    }
}
