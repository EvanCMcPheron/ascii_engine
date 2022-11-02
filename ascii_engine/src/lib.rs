pub mod app;
pub mod entity;
pub mod sprite;

pub mod prelude {
    pub use crate::app::App;
    pub use crate::entity::{Behaviour, Ent};
    pub use crate::sprite::Sprite;
}