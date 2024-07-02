#[derive(Debug, Clone)]
pub struct Layout {
    pub chars: Vec<char>
}

impl Layout {
    pub fn new(chars: &[char]) -> Self {
        Self {
            chars: chars.to_vec()
        }
    }
}

impl From<Vec<char>> for Layout {
    fn from(chars: Vec<char>) -> Self {
        Self { chars }
    }
}
