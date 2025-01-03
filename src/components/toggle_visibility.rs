use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Clone)]
#[require(Button)]
pub struct ToggleVisibility {
	pub toggles: Vec<Entity>,
	pub visible: bool,
}

impl ToggleVisibility {
	pub fn apply(
		mut commands: Commands,
		toggles: Query<&ToggleVisibility, Changed<ToggleVisibility>>,
	) {
		for toggle in &toggles {
			for target in &toggle.toggles {
				let Some(mut target) = commands.get_entity(*target) else {
					continue;
				};
				match toggle.visible {
					true => target.try_insert(Visibility::Inherited),
					false => target.try_insert(Visibility::Hidden),
				};
			}
		}
	}

	pub fn toggle(mut buttons: Query<(&Interaction, &mut ToggleVisibility), Changed<Interaction>>) {
		for (button, mut toggle) in &mut buttons {
			if button != &Interaction::Pressed {
				continue;
			}

			toggle.visible = !toggle.visible;
		}
	}
}
