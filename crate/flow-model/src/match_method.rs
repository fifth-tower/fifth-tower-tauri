use serde::{Deserialize, Serialize};

//匹配图像方法，对应MatchTemplateMethod
#[derive(Default, Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum MatchMethod {
    #[default]
    ///SumOfSquaredErrors
    SOSE,
    ///SumOfSquaredErrorsNormalized
    SOSEN,
    ///CrossCorrelation
    CC,
    ///CrossCorrelationNormalized
    CCN,
}
impl ToString for MatchMethod {
    fn to_string(&self) -> String {
        match self {
            MatchMethod::SOSE => "SumOfSquaredErrors".into(),
            MatchMethod::SOSEN => "SumOfSquaredErrors".into(),
            MatchMethod::CC => "SumOfSquaredErrors".into(),
            MatchMethod::CCN => "SumOfSquaredErrors".into(),
        }
    }
}

impl MatchMethod {
    pub fn is_matched(&self, quota: f32, result: f32) -> bool {
        match self {
            Self::SOSE | Self::SOSEN => result <= quota,
            Self::CC | Self::CCN => result >= quota,
        }
    }
    pub fn tip(&self) -> String {
        match self {
            MatchMethod::SOSE | MatchMethod::SOSEN => {
                "实际值小于参考值表示匹配，越小匹配度越高".into()
            }
            MatchMethod::CC | MatchMethod::CCN => "实际值大于参考值表示匹配，越大匹配度越高".into(),
        }
    }
}
