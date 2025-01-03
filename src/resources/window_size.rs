use bevy::prelude::*;

#[derive(Resource, Debug, PartialEq, Default, Clone, Copy)]
pub struct WindowSize {
	height: f32,
	width: f32,
}

impl WindowSize {
	pub fn height(&self) -> f32 {
		self.height
	}

	pub fn width(&self) -> f32 {
		self.width
	}

	pub fn initialize(mut commands: Commands) {
		commands.init_resource::<WindowSize>();
	}

	pub fn update(mut window_size: ResMut<WindowSize>, windows: Query<&Window, Changed<Window>>) {
		let Ok(main_window) = windows.get_single() else {
			return;
		};

		if window_size.matches(main_window) {
			return;
		}

		*window_size = WindowSize {
			width: main_window.width(),
			height: main_window.height(),
		};
	}

	fn matches(&self, window: &Window) -> bool {
		window.width() == self.width && window.height() == self.height
	}
}
