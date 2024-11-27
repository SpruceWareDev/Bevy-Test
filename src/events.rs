use bevy::{prelude::*, transform};
use crate::game::{Bullet, Collider, Health};

#[derive(Event)]
pub struct BulletCollisionEvent {
    bullet: Bullet,
    damage: i8,
    object_heath: Health,
}