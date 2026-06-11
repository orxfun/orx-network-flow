use crate::graphs::visualization::dot::{vertex_shape::VertexShape, vertex_style::VertexStyle};
use alloc::string::String;
use core::fmt::Display;

#[derive(Default, Clone)]
pub struct VertexSettings {
    pub shape: Option<VertexShape>,
    pub style: Option<VertexStyle>,
    pub fill_color: Option<String>,
}

impl Display for VertexSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (&self.shape, &self.style, &self.fill_color) {
            (None, None, None) => Ok(()),
            _ => {
                if let Some(shape) = &self.shape {
                    write!(f, ", shape={}", shape)?;
                }
                if let Some(style) = &self.style {
                    write!(f, ", style={}", style)?;
                }
                if let Some(fill_color) = &self.fill_color {
                    write!(f, ", fillcolor={}", fill_color)?;
                }
                Ok(())
            }
        }
    }
}
