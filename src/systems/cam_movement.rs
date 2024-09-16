use crate::traits::movement::AnchoredMovement;
use bevy::prelude::*;

pub fn cam_movement<TEvent: AnchoredMovement + Event>() {}

#[cfg(test)]
mod tests {
	use super::*;
}
