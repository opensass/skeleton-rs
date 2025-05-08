#[derive(Clone, PartialEq, Default)]
pub enum Variant {
    #[default]
    Text,
    Circular,
    Rectangular,
    Rounded,
    Image,
    Avatar,
    Button,
}

#[derive(Clone, PartialEq, Default)]
pub enum Animation {
    #[default]
    Pulse,
    Wave,
    None,
}

#[derive(Clone, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Custom(&'static str),
}

#[derive(Clone, PartialEq, Default)]
pub enum Direction {
    #[default]
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
    CustomAngle(i64),
}
