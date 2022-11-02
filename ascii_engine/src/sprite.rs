


#[derive(Debug, Clone)]
pub struct Sprite {
    chars: Box<[Box<[char]>]>,
    dimensions: (u32, u32),
}

impl std::default::Default for Sprite {
    fn default() -> Sprite {
        Sprite {
            chars: Box::new([]),
            dimensions: (0, 0),
        }
    }
}
