use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rect {
    /// The x coordinate of the top left corner.
    pub x: i32,
    /// The y coordinate of the top left corner.
    pub y: i32,
    /// The rectangle's width.
    pub width: u32,
    /// The rectangle's height.
    pub height: u32,
}
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn to_relative(&self, x: i32, y: i32) -> Self {
        Self {
            x: self.x - x,
            y: self.y - y,
            width: self.width,
            height: self.height,
        }
    }

    ///范围超过应用窗口外时返回true
    pub fn adjust_overflow(&mut self, dim: (u32, u32)) -> bool {
        if self.width > dim.0 {
            self.width = dim.0;
        }
        if self.height > dim.1 {
            self.height = dim.1;
        }
        let mut overflow = false;
        if self.x + self.width as i32 > dim.0 as i32 {
            self.x = dim.0 as i32 - self.width as i32;
            overflow = true;
        }
        if self.y + self.height as i32 > dim.1 as i32 {
            self.y = dim.1 as i32 - self.height as i32;
            overflow = true;
        }
        overflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_negative() {
        let i = 1_i32;
        println!("{}", i * -1);
        println!("{}", 0 - i);
        println!("{}", i.wrapping_neg());
    }
}
