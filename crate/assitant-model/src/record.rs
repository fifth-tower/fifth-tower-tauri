use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckPoint {
    AppClosed,
    Minimized,
    WindowSize,
    AdjustSize,
}
impl ToString for CheckPoint {
    fn to_string(&self) -> String {
        match self {
            Self::AppClosed => "应用不能关闭",
            Self::Minimized => "窗口不能最小化",
            Self::WindowSize => "窗口与之前录制时大小一样",
            Self::AdjustSize => "调整窗口大小与之前一样",
        }
        .into()
    }
}
#[cfg(test)]
mod tests {
    use tracing::debug;

    use crate::tests::init;

    use super::*;

    #[test]
    fn it_works() {
        init();

        debug!("{:?}", CheckPoint::AppClosed);
    }
}
