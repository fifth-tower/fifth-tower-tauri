use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActionCommand {
    //ImageArea,template,width,height,rel-x,rel-y,, MatchMethod, match_value
    Image(Rect, String, u32, u32, i32, i32, MatchMethod, f32),
    //x,y,refer
    Move(i32, i32, String),
    Click(i32, i32, String),
    //x,y,refer
    Input(i32, i32, String),
    //x,y
    KeyCombi(i32, i32),
    //len,is_vertical,x,y,refer
    Scroll(i32, bool, i32, i32, String),
    ///用于模拟刮刮乐操作
    GuaGuaLe(Rect, String),
    IncludeFlow,
    SubFlowStart,
    SubFlowEnd,
}

impl ActionCommand {
    ///用于获取refer图片大小
    pub const CURSOR_REF_OFFSET: (u32, u32) = (80, 30);
    pub fn message(&self, correct_coordinate: bool) -> String {
        match self {
            Self::Image(_, template, ..) => {
                if correct_coordinate {
                    format!("匹配图片：{},并点击", template)
                } else {
                    format!("匹配图片,并点击")
                }
            }
            Self::GuaGuaLe(rect, _) => {
                if correct_coordinate {
                    format!("刮一刮：{:?}", rect)
                } else {
                    format!("刮一刮")
                }
            }
            Self::Move(x, y, _) => {
                if correct_coordinate {
                    format!("移动鼠标到：({},{})", x, y)
                } else {
                    format!("移动鼠标到：({},{})", "x", "y")
                }
            }
            Self::Click(x, y, _) => {
                if correct_coordinate {
                    format!("移动鼠标到：({},{}),并点击", x, y)
                } else {
                    format!("移动鼠标到：({},{}),并点击", "x", "y")
                }
            }
            Self::Scroll(_, is_vertical, ..) => {
                if correct_coordinate {
                    format!(
                        "{}滚动鼠标",
                        if *is_vertical {
                            "垂直方向"
                        } else {
                            "水平方向"
                        }
                    )
                } else {
                    "滚动鼠标".into()
                }
            }
            Self::Input(x, y, _) => {
                if correct_coordinate {
                    format!("在位置({},{})输入文字", x, y)
                } else {
                    format!("在位置(x,y)输入文字")
                }
            }
            Self::KeyCombi(x, y, ..) => {
                if correct_coordinate {
                    format!("在位置({},{})按下组合键", x, y)
                } else {
                    format!("在位置(x,y)按下组合键")
                }
            }
            Self::IncludeFlow => "执行一个导入流程".into(),
            Self::SubFlowStart => "开始子流程".into(),
            Self::SubFlowEnd => "结束子流程".into(),
        }
    }

    pub fn of(name: &str) -> Self {
        match name {
            "Move" => Self::Move(0, 0, "".into()),
            "Click" => Self::Click(0, 0, "".into()),
            "Image" => Self::Image(
                Rect::default(),
                "".into(),
                0,
                0,
                0,
                0,
                MatchMethod::default(),
                0.0,
            ),
            "Scroll" => Self::Scroll(0, true, 0, 0, "".into()),
            "GuaGuaLe" => Self::GuaGuaLe(Rect::default(), "".into()),
            "Input" => Self::Input(0, 0, "".into()),
            "KeyCombi" => Self::KeyCombi(0, 0),
            "IncludeFlow" => Self::IncludeFlow,
            "SubFlowStart" => Self::SubFlowStart,
            "SubFlowEnd" => Self::SubFlowEnd,
            _ => todo!(),
        }
    }
}
