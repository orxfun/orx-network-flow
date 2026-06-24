use alloc::string::String;
use core::fmt::Display;

#[derive(Default, Clone)]
pub struct EdgeSettings {
    pub color: Option<String>,
    pub pen_width: Option<f64>,
}

impl Display for EdgeSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(color) = &self.color {
            write!(f, ", color={}", color)?;
        }
        if let Some(pen_width) = &self.pen_width {
            write!(f, ", penwidth={}", pen_width)?;
        }
        Ok(())
    }
}
