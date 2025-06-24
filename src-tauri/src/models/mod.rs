//! Models module - re-exports from entity

pub use crate::entity::daily_checkins;
pub use crate::entity::micro_tasks;

pub mod prelude {
    pub use super::daily_checkins;
    pub use super::micro_tasks;
}
