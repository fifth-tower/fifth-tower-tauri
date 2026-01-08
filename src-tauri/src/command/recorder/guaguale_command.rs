use super::*;
use flow_model::{ActionCommand, Rect};
use image::{GenericImage, ImageBuffer, Rgba};
use recorder_util::get_guaguale_rect;
use tokio::sync::oneshot;
use tower::common_rs::now_secs;
use tracing::warn;
use xcap::Monitor;

pub(crate) struct GuaGuaLeCommand;
impl GuaGuaLeCommand {
    pub(crate) fn fill_command(
        app_id: &str,
        flow_id: &str,
        cursor: (i32, i32),
    ) -> Option<ActionCommand> {
        let (tx, rx) = oneshot::channel::<ImageBuffer<Rgba<u8>, Vec<u8>>>();

        tauri::async_runtime::spawn(async move {
            let monitor = Monitor::from_point(cursor.0, cursor.1).unwrap();
            tx.send(monitor.capture_image().unwrap()).unwrap();
        });
        get_guaguale_rect().map(|(rect, (monitor_x, monitor_y))| {
            let image_name = format!("{}/shot-{}.png", flow_id, now_secs());
            let mut window_image = rx.blocking_recv().unwrap();

            let dim = window_image.dimensions();
            let mut rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
            if rect.adjust_overflow((monitor_x as u32 + dim.0, monitor_y as u32 + dim.1)) {
                warn!("overflow occur rect={:?},dim={:?}", rect, dim);
            }
            let image = window_image
                .sub_image(
                    (rect.x - monitor_x) as u32,
                    (rect.y - monitor_y) as u32,
                    rect.width,
                    rect.height,
                )
                .to_image();

            save_image(app_id, &image_name, &image);

            ActionCommand::GuaGuaLe(
                Rect::new(rect.x, rect.y, rect.width, rect.height),
                image_name,
            )
        })
    }
}
