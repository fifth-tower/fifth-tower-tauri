use super::*;
use flow::match_by_template;
use flow_model::{ActionCommand, MatchMethod, Rect};
use image::{DynamicImage, GenericImage, ImageBuffer, Rgba};
use recorder_util::get_image_rects;
use tokio::sync::oneshot;
use tower::common_rs::now_secs;
use tracing::{debug, warn};
use xcap::Monitor;

pub(crate) struct ImageCommand;
impl ImageCommand {
    pub fn fill_command(app_id: &str, flow_id: &str, cursor: (i32, i32)) -> Option<ActionCommand> {
        let (tx, rx) = oneshot::channel::<ImageBuffer<Rgba<u8>, Vec<u8>>>();

        tauri::async_runtime::spawn(async move {
            let monitor = Monitor::from_point(cursor.0, cursor.1).unwrap();
            tx.send(monitor.capture_image().unwrap()).unwrap();
        });
        get_image_rects().map(|(rect, template, (monitor_x, monitor_y))| {
            let image_name = format!("{}/shot-{}.png", flow_id, now_secs());
            let mut window_image = rx.blocking_recv().unwrap();

            let dim = window_image.dimensions();
            let mut rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
            if rect.adjust_overflow((monitor_x as u32 + dim.0, monitor_y as u32 + dim.1)) {
                warn!("overflow occur rect={:?},dim={:?}", rect, dim);
            }
            let mut template = Rect::new(template.x, template.y, template.width, template.height);
            if template.adjust_overflow((monitor_x as u32 + dim.0, monitor_y as u32 + dim.1)) {
                warn!("overflow occur template={:?},dim={:?}", template, dim);
            }
            let template_image = window_image
                .sub_image(
                    (template.x - monitor_x) as u32,
                    (template.y - monitor_y) as u32,
                    template.width,
                    template.height,
                )
                .to_image();

            let template_image = save_image(app_id, &image_name, &template_image);

            let (match_method, match_value) = get_match_value(
                &template_image,
                &mut window_image,
                &rect.to_relative(monitor_x, monitor_y),
            );

            ActionCommand::Image(
                rect,
                image_name,
                template.width,
                template.height,
                0,
                0,
                match_method,
                match_value,
            )
        })
    }
}

fn get_match_value(
    template_image: &str,
    window_image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    rect: &Rect,
) -> (flow_model::MatchMethod, f32) {
    let match_method = MatchMethod::default();
    let source = window_image
        .sub_image(rect.x as u32, rect.y as u32, rect.width, rect.height)
        .to_image();

    let template_image = image::open(template_image)
        .expect(format!("Failed to open template image:{}", template_image).as_str());
    let (min_value, _) = match_by_template(
        DynamicImage::ImageRgba8(source),
        template_image,
        match_method,
    );
    (match_method, min_value)
}
