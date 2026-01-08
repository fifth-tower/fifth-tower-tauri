use flow_model::ActionCommand;
use tower::common_rs::now_secs;

use crate::command::recorder::capture_refer_image;

use super::save_image;

pub(crate) struct InputCommand;
impl InputCommand {
    pub(crate) fn fill_command(
        app_id: &str,
        flow_id: &str,
        cursor: (i32, i32),
    ) -> Option<ActionCommand> {
        let image_name = format!("{}/shot-{}.png", flow_id, now_secs());
        let image = capture_refer_image(cursor);

        save_image(app_id, &image_name, &image);

        Some(ActionCommand::Input(cursor.0, cursor.1, image_name))
    }
}
