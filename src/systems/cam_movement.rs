use crate::traits::movement::{Anchor, AnchoredMovement, Seconds};
use bevy::prelude::*;

pub fn cam_movement<TEvent: AnchoredMovement + Event>(
	time: Res<Time<Real>>,
	mut cameras: Query<&mut Transform, With<Camera>>,
	mut events: EventReader<TEvent>,
) {
	let around = Anchor(Vec3::new(0., 0.5, 0.));
	let delta = Seconds(time.delta_seconds());

	for event in events.read() {
		apply_event_transformations(&mut cameras, event, around, delta);
	}
}

fn apply_event_transformations<TEvent: AnchoredMovement>(
	cameras: &mut Query<&mut Transform, With<Camera>>,
	event: &TEvent,
	around: Anchor,
	delta: Seconds,
) {
	for mut transform in cameras {
		event.anchored_movement(transform.as_mut(), around, delta);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::traits::movement::{Anchor, Seconds};
	use bevy::ecs::system::RunSystemOnce;
	use mockall::{mock, predicate::eq};
	use std::time::{Duration, Instant};

	#[derive(Event)]
	struct MyEvent {
		mock: MockMyEvent,
	}

	impl MyEvent {
		fn with_mock(mut setup: impl FnMut(&mut MockMyEvent)) -> Self {
			let mut mock = MockMyEvent::default();
			setup(&mut mock);

			Self { mock }
		}
	}

	#[automock]
	impl AnchoredMovement for MyEvent {
		fn anchored_movement(&self, agent: &mut Transform, around: Anchor, delta: Seconds) {
			self.mock.anchored_movement(agent, around, delta);
		}
	}

	fn tick_time(app: &mut App, delta: Duration) {
		let mut time = app.world_mut().resource_mut::<Time<Real>>();
		let last_update = time.last_update().unwrap_or_else(Instant::now);
		time.update_with_instant(last_update + delta);
	}

	fn setup() -> App {
		let mut app = App::new();
		app.world_mut().init_resource::<Time<Real>>();
		app.add_event::<MyEvent>();

		tick_time(&mut app, Duration::ZERO);
		app
	}

	#[test]
	fn apply_anchored_movement() {
		let mut app = setup();
		app.world_mut()
			.spawn((Transform::from_xyz(1., 2., 3.), Camera::default()));

		tick_time(&mut app, Duration::from_secs(42));
		app.world_mut().send_event(MyEvent::with_mock(assert));
		app.world_mut().run_system_once(cam_movement::<MyEvent>);

		fn assert(mock: &mut MockMyEvent) {
			mock.expect_anchored_movement()
				.with(
					eq(Transform::from_xyz(1., 2., 3.)),
					eq(Anchor(Vec3::new(0., 0.5, 0.))),
					eq(Seconds(42.)),
				)
				.times(1)
				.return_const(());
		}
	}

	#[test]
	fn do_not_apply_anchored_movement_when_not_camera_present() {
		let mut app = setup();
		app.world_mut().spawn(Transform::from_xyz(1., 2., 3.));

		tick_time(&mut app, Duration::from_secs(42));
		app.world_mut().send_event(MyEvent::with_mock(assert));
		app.world_mut().run_system_once(cam_movement::<MyEvent>);

		fn assert(mock: &mut MockMyEvent) {
			mock.expect_anchored_movement().never().return_const(());
		}
	}

	#[test]
	fn apply_anchored_movement_for_multiple_cameras() {
		let mut app = setup();
		app.world_mut()
			.spawn((Transform::from_xyz(1., 2., 3.), Camera::default()));
		app.world_mut()
			.spawn((Transform::from_xyz(4., 5., 6.), Camera::default()));

		tick_time(&mut app, Duration::from_secs(11));
		app.world_mut().send_event(MyEvent::with_mock(assert));
		app.world_mut().run_system_once(cam_movement::<MyEvent>);

		fn assert(mock: &mut MockMyEvent) {
			mock.expect_anchored_movement()
				.with(
					eq(Transform::from_xyz(1., 2., 3.)),
					eq(Anchor(Vec3::new(0., 0.5, 0.))),
					eq(Seconds(11.)),
				)
				.times(1)
				.return_const(());
			mock.expect_anchored_movement()
				.with(
					eq(Transform::from_xyz(4., 5., 6.)),
					eq(Anchor(Vec3::new(0., 0.5, 0.))),
					eq(Seconds(11.)),
				)
				.times(1)
				.return_const(());
		}
	}
}
