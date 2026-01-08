use tao::dpi::PhysicalPosition;

pub fn to_point(pos: PhysicalPosition<f64>) -> (i32, i32) {
    (pos.x as i32, pos.y as i32)
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn new(lt: (i32, i32), rb: (i32, i32)) -> Self {
        let x = if lt.0 < rb.0 { lt.0 } else { rb.0 };
        let y = if lt.1 < rb.1 { lt.1 } else { rb.1 };
        Self {
            x,
            y,
            width: (rb.0 - lt.0).abs() as u32,
            height: (rb.1 - lt.1).abs() as u32,
        }
    }
}
