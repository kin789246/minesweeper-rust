use crate::components::Coordinates;
use bevy::ecs::event::Event;

#[derive(Event, Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Event, Debug, Copy, Clone)]
pub struct TileMarkEvent(pub Coordinates);

#[derive(Event, Debug, Copy, Clone)]
pub struct BoardCompletedEvent;

#[derive(Event, Debug, Copy, Clone)]
pub struct BombExplosionEvent;
