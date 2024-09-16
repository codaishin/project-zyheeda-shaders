use bevy::{input::ButtonInput, prelude::*};
use std::hash::Hash;

pub fn holding_button<TButton>(button: TButton) -> impl Fn(Res<ButtonInput<TButton>>) -> bool
where
	TButton: Copy + Eq + Hash + Sync + Send + 'static,
{
	move |input: Res<ButtonInput<TButton>>| input.pressed(button)
}

#[cfg(test)]
mod tests {
	use super::*;
	use bevy::ecs::system::RunSystemOnce;

	#[derive(Clone, Copy, PartialEq, Eq, Hash)]
	enum MyButton {
		Left,
		Right,
	}

	fn setup() -> App {
		let mut app = App::new();
		app.init_resource::<ButtonInput<MyButton>>();

		app
	}

	#[test]
	fn holding_button_true() {
		let mut app = setup();
		app.world_mut()
			.resource_mut::<ButtonInput<MyButton>>()
			.press(MyButton::Left);

		let is_holding = app
			.world_mut()
			.run_system_once(holding_button(MyButton::Left));

		assert!(is_holding);
	}

	#[test]
	fn holding_button_false() {
		let mut app = setup();
		app.world_mut()
			.resource_mut::<ButtonInput<MyButton>>()
			.press(MyButton::Right);

		let is_holding = app
			.world_mut()
			.run_system_once(holding_button(MyButton::Left));

		assert!(!is_holding);
	}
}
