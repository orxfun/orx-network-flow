use core::fmt::Display;

#[derive(Clone)]
pub enum NodeStyle {
    Filled,
    Invisible,
    Diagonals,
    Rounded,
    Dashed,
    Dotted,
    Solid,
    Bold,
}

impl Display for NodeStyle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let style = match self {
            Self::Filled => "filled",
            Self::Invisible => "invisible",
            Self::Diagonals => "diagonals",
            Self::Rounded => "rounded",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
            Self::Solid => "solid",
            Self::Bold => "bold",
        };

        f.write_str(style)
    }
}
