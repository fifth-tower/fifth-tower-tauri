use flow_model::{Action, Flow};
use tower::common_rs::copy_file;

use crate::Dir;

///调整粘贴过来的图片路径
pub fn adjust_image(flow: &mut Flow, app_id: &str, dir: &Dir) {
    let copy_image = |source: &mut String| {
        let mut image = source.split("/");
        let flow_id = image.nth(0).unwrap();
        let image_name = image.nth(0).unwrap();
        if flow_id == flow.flow_id {
            return;
        }
        //copy image
        let app_dir = dir.app(app_id);
        copy_file(
            format!("{}/{}", app_dir, source),
            format!("{}/{}", app_dir, flow.flow_id),
            true,
        );

        *source = format!("{}/{}", flow.flow_id, image_name);
    };
    for action in &mut flow.actions {
        match &mut action.0 {
            Action::Image(_, template, ..) => copy_image(template),
            Action::Move(.., image) | Action::Click(.., image) => {
                if image.len() > 0 {
                    copy_image(image)
                }
            }
            _ => {}
        };
    }
}

///resize时使用
#[inline]
pub fn is_same_size(win: u32, app: u32) -> bool {
    (win as i32 - app as i32).abs() <= 1
}
