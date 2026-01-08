use flow_model::ActionCommand;
use image::{ImageBuffer, Rgba};
use recorder_util::get_scroll;
use tokio::sync::oneshot;
use tower::common_rs::now_secs;
use tracing::debug;

use crate::command::recorder::{async_capture_refer_image, save_image};

pub(crate) struct ScrollCommand;
impl ScrollCommand {
    pub fn fill_command(app_id: &str, flow_id: &str, cursor: (i32, i32)) -> Option<ActionCommand> {
        let (tx, rx) = oneshot::channel::<ImageBuffer<Rgba<u8>, Vec<u8>>>();

        async_capture_refer_image(tx, cursor);

        get_scroll().map(|scroll| {
            debug!("get_scroll:cursor:{:?}", cursor);

            let image_name = format!("{}/shot-{}.png", flow_id, now_secs());
            let image = rx.blocking_recv().unwrap();

            save_image(app_id, &image_name, &image);

            if scroll.0 == 0 {
                ActionCommand::Scroll(scroll.1, true, cursor.0, cursor.1, image_name)
            } else {
                ActionCommand::Scroll(scroll.0, false, cursor.0, cursor.1, image_name)
            }
        })
    }
}
