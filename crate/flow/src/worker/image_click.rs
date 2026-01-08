use crate::{match_by_template, FlowExecutor, ImageMatchInfo, MouseClickWorker};
use enigo::Coordinate;
use flow_model::ImageClickAtrr;
use image::{imageops::FilterType, DynamicImage, GenericImage};
use tracing::{info, warn};

pub(crate) struct ImageClickWorker;

impl ImageClickWorker {
    pub fn do_work(executor: &mut FlowExecutor, attr: &ImageClickAtrr) -> (bool, ImageMatchInfo) {
        let &ImageClickAtrr {
            match_area,
            ref template,
            click_pos: (rel_x, rel_y),
            match_method,
            match_value,
            ..
        } = attr;
        //选取rect
        let mut image = executor.window().capture_image().unwrap();

        //选取rect
        let fact_match_area = executor.transfer_rect(&match_area, Coordinate::Rel);
        let image = image
            .sub_image(
                fact_match_area.x as u32,
                fact_match_area.y as u32,
                fact_match_area.width,
                fact_match_area.height,
            )
            .to_image();
        let source_image = DynamicImage::ImageRgba8(image).resize(
            match_area.width,
            match_area.height,
            FilterType::Gaussian,
        );

        //选取template
        let template_image = executor.get_template_image(template);
        //匹配
        let match_info = match_by_template(source_image, template_image, match_method);
        let matched = if !match_method.is_matched(match_value, match_info.0) {
            info!(
                "template:{},匹配失败,match_value={},match_result_value={}",
                template, match_value, match_info.0
            );

            false
        } else {
            MouseClickWorker::do_work(
                executor,
                (
                    match_area.x + match_info.1 .0 as i32 + rel_x,
                    match_area.y + match_info.1 .1 as i32 + rel_y,
                ),
            )
        };
        (
            matched,
            (
                match_info.0,
                (
                    match_area.x as u32 + match_info.1 .0,
                    match_area.y as u32 + match_info.1 .1,
                ),
            ),
        )
    }
}
