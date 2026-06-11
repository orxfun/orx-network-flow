use alloc::string::String;
use core::fmt::Display;

#[derive(Default, Clone)]
pub struct EdgeSettings {
    pub color: Option<String>,
}

impl Display for EdgeSettings {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(color) = &self.color {
            write!(f, ", color={}", color)?;
        }
        Ok(())
    }
}
