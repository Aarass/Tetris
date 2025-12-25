use crate::Direction;
use bevy::ecs::message::Message;

#[derive(Message)]
pub struct MovePiece(pub Direction);
