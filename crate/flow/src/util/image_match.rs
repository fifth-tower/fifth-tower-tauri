use std::time::Instant;

use flow_model::MatchMethod;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use imageproc::template_matching::{
    find_extremes, match_template_parallel, Extremes, MatchTemplateMethod,
};
use tracing::debug;

///(match_result_value, location)
pub type ImageMatchInfo = (f32, (u32, u32));

pub fn match_by_template(
    source: DynamicImage,
    template: DynamicImage,
    match_method: MatchMethod,
) -> ImageMatchInfo {
    let start = Instant::now();

    let ret = match_template_internal(source, template, match_method);

    debug!(
        "match_by_template 耗时：{:?}, ret:{:?}",
        start.elapsed(),
        ret
    );
    ret
}

fn match_template_internal(
    source: DynamicImage,
    template: DynamicImage,
    match_method: MatchMethod,
) -> ImageMatchInfo {
    let source_dim = source.dimensions();
    let template_dim = template.dimensions();

    let scale = 2;
    // Convert images to grayscale
    let source_gray = source
        .resize(
            source_dim.0 / scale,
            source_dim.1 / scale,
            FilterType::Nearest,
        )
        .to_luma8();
    let template_gray = template
        .resize(
            template_dim.0 / scale,
            template_dim.1 / scale,
            FilterType::Nearest,
        )
        .to_luma8();
    // Perform template matching
    let result = match_template_parallel(
        &source_gray,
        &template_gray,
        to_match_template_method(match_method),
    );

    debug!("find_extremes:{:?}", find_extremes(&result));
    // Find the best match location
    let Extremes {
        min_value_location,
        min_value,
        ..
    } = find_extremes(&result);

    (
        min_value,
        (min_value_location.0 * scale, min_value_location.1 * scale),
    )
}

fn to_match_template_method(flow_match_method: MatchMethod) -> MatchTemplateMethod {
    match flow_match_method {
        MatchMethod::SOSE => MatchTemplateMethod::SumOfSquaredErrors,
        MatchMethod::SOSEN => MatchTemplateMethod::SumOfSquaredErrorsNormalized,
        MatchMethod::CC => MatchTemplateMethod::CrossCorrelation,
        MatchMethod::CCN => MatchTemplateMethod::CrossCorrelationNormalized,
    }
}

#[cfg(test)]
mod tests {
    use flow_model::Rect;
    use image::GenericImage;
    use tracing::Level;

    use crate::get_window;

    use super::*;

    fn init() {
        tracing_subscriber::fmt()
            .with_max_level(Level::DEBUG)
            .init();
    }
    #[test]
    fn test_match_by_template() {
        init();
        let window = get_window(17096).unwrap();
        //选取rect
        let mut image = window.capture_image().unwrap();

        let match_area = Rect::new(301, 133, 640, 360);
        //选取rect
        let image = image
            .sub_image(
                match_area.x as u32,
                match_area.y as u32,
                match_area.width,
                match_area.height,
            )
            .to_image();
        let source_image = DynamicImage::ImageRgba8(image);

        let template_image = "C:/Users/gelye/AppData/Roaming/nicee/resource/record/FQAAAAAAAADmoqblubvopbmuLjvvJrml7bnqbo=/flow-1749111619/shot-1749186754.png";
        let template_image = image::open(template_image)
            .expect(format!("Failed to open template image:{}", template_image).as_str());

        match_by_template(source_image, template_image, MatchMethod::SOSE);
    }
}
