use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageClickAtrr {
    ///选取匹配范围
    pub match_area: Rect,
    //模板图片
    pub template: String,
    //模板大小
    pub template_size: (u32, u32),
    //点击的坐标。以匹配的图片坐标为起点
    pub click_pos: (i32, i32),
    //匹配模板方法
    pub match_method: MatchMethod,
    //匹配标准值
    pub match_value: f32,
}

impl TryFrom<&Action> for ImageClickAtrr {
    type Error = String;

    fn try_from(value: &Action) -> Result<Self, Self::Error> {
        let &Action::Image(
            match_area,
            ref template,
            width,
            height,
            rel_x,
            rel_y,
            match_method,
            match_value,
        ) = value
        else {
            return Err("action非KeyCombi".to_string());
        };
        Ok(Self {
            match_area,
            template: template.clone(),
            template_size: (width, height),
            click_pos: (rel_x, rel_y),
            match_method,
            match_value,
        })
    }
}

impl TryFrom<Action> for ImageClickAtrr {
    type Error = String;

    fn try_from(value: Action) -> Result<Self, Self::Error> {
        ImageClickAtrr::try_from(&value)
    }
}
