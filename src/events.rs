use bevy::prelude::*;
use crate::game::{Bullet, Health};

#[derive(Event)]
pub struct BulletCollisionEvent {
    bullet: Bullet,
    damage: i8,
    object_heath:  Health,
}