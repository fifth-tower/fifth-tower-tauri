use flow::Dir;
use flow_model::ActionCommand;
use image::{GenericImage, ImageBuffer, Rgba};
use tokio::sync::oneshot::Sender;
use tracing::debug;
use xcap::Monitor;

use super::*;

pub trait ActionRecorder {
    fn fill_command(
        &self,
        app_id: &str,
        flow_id: &str,
        cursor: (i32, i32),
    ) -> Option<ActionCommand>;
}

impl ActionRecorder for ActionCommand {
    fn fill_command(
        &self,
        app_id: &str,
        flow_id: &str,
        cursor: (i32, i32),
    ) -> Option<ActionCommand> {
        match self {
            ActionCommand::Image(..) => ImageCommand::fill_command(app_id, flow_id, cursor),
            ActionCommand::Move(..) => MoveCommand::fill_command(app_id, flow_id, cursor, false),
            ActionCommand::Click(..) => MoveCommand::fill_command(app_id, flow_id, cursor, true),
            ActionCommand::Scroll(..) => ScrollCommand::fill_command(app_id, flow_id, cursor),
            ActionCommand::GuaGuaLe(..) => GuaGuaLeCommand::fill_command(app_id, flow_id, cursor),
            ActionCommand::KeyCombi(..) => KeyCombiCommand::fill_command(app_id, flow_id, cursor),
            ActionCommand::Input(..) => InputCommand::fill_command(app_id, flow_id, cursor),
            _ => Some(self.to_owned()),
        }
    }
}

pub(crate) fn async_capture_refer_image(
    tx: Sender<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    cursor: (i32, i32),
) {
    tauri::async_runtime::spawn(async move {
        let image = capture_refer_image(cursor);
        tx.send(image).unwrap();
    });
}

pub(crate) fn capture_refer_image(cursor: (i32, i32)) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let monitor = Monitor::from_point(cursor.0, cursor.1).unwrap();
    let mut image = monitor.capture_image().unwrap();

    let offset = ActionCommand::CURSOR_REF_OFFSET;
    let (x, y) = (
        cursor.0 - monitor.x().unwrap(),
        cursor.1 - monitor.y().unwrap(),
    );
    image
        .sub_image(
            if x as u32 > offset.0 {
                x as u32 - offset.0
            } else {
                0
            },
            if y as u32 > offset.1 {
                y as u32 - offset.1
            } else {
                0
            },
            offset.0 * 2,
            offset.1 * 2,
        )
        .to_image()
}

pub(crate) fn save_image(
    app_id: &str,
    image_name: &str,
    image: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> String {
    let path = Dir::Record.image(app_id, Some(image_name));
    debug!("save image to: {}", path.0);
    image.save(&path.0).unwrap();
    path.0
}
