
pub enum Alignment {
    Left,
    Right,
}

pub struct Formatting {
    pub padding: usize,
    pub alignment: Alignment,
    pub max_width: usize,
    pub padding_left: Option<usize>,
    pub padding_right: Option<usize>,
    pub padding_top: Option<usize>,
    pub padding_bottom: Option<usize>,
}

impl Formatting {
    pub fn new() -> Formatting {
        Formatting {
            padding: 2,
            alignment: Alignment::Left,
            max_width: 80,
            padding_left: None,
            padding_right: None,
            padding_top: None,
            padding_bottom: None,
        }
    }
}
