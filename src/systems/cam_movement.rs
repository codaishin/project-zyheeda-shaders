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
	use std::time::{Duration, Instant};

	fn tick_time(app: &mut App, delta: Duration) {
		let mut time = app.world_mut().resource_mut::<Time<Real>>();
		let last_update = time.last_update().unwrap_or_else(Instant::now);
		time.update_with_instant(last_update + delta);
	}

	fn setup<TEvent: Event>() -> App {
		let mut app = App::new();
		app.world_mut().init_resource::<Time<Real>>();
		app.add_event::<TEvent>();

		tick_time(&mut app, Duration::ZERO);
		app
	}

	#[test]
	fn apply_anchored_movement() {
		#[derive(Event)]
		struct MyEvent;

		static mut CALL_ARGS: Vec<(Transform, Anchor, Seconds)> = vec![];

		impl AnchoredMovement for MyEvent {
			fn anchored_movement(&self, agent: &mut Transform, anchor: Anchor, delta: Seconds) {
				unsafe {
					CALL_ARGS.push((*agent, anchor, delta));
				}
			}
		}

		let mut app = setup::<MyEvent>();
		app.world_mut().spawn(Camera3dBundle {
			transform: Transform::from_xyz(1., 2., 3.),
			..default()
		});

		tick_time(&mut app, Duration::from_secs(42));
		app.world_mut().send_event(MyEvent);
		app.world_mut().run_system_once(cam_movement::<MyEvent>);

		assert_eq!(
			vec![(
				Transform::from_xyz(1., 2., 3.),
				Anchor(Vec3::new(0., 0.5, 0.)),
				Seconds(42.)
			)],
			unsafe { CALL_ARGS.clone() }
		)
	}

	#[test]
	fn do_not_apply_anchored_movement_when_not_camera_present() {
		#[derive(Event)]
		struct MyEvent;

		static mut CALL_ARGS: Vec<(Transform, Anchor, Seconds)> = vec![];

		impl AnchoredMovement for MyEvent {
			fn anchored_movement(&self, agent: &mut Transform, anchor: Anchor, delta: Seconds) {
				unsafe {
					CALL_ARGS.push((*agent, anchor, delta));
				}
			}
		}

		let mut app = setup::<MyEvent>();
		app.world_mut().spawn(Transform::from_xyz(1., 2., 3.));

		tick_time(&mut app, Duration::from_secs(42));
		app.world_mut().send_event(MyEvent);
		app.world_mut().run_system_once(cam_movement::<MyEvent>);

		assert_eq!(vec![] as Vec<(Transform, Anchor, Seconds)>, unsafe {
			CALL_ARGS.clone()
		})
	}

	#[test]
	fn apply_anchored_movement_for_multiple_cameras() {
		#[derive(Event)]
		struct MyEvent;

		static mut CALL_ARGS: Vec<(Transform, Anchor, Seconds)> = vec![];

		impl AnchoredMovement for MyEvent {
			fn anchored_movement(&self, agent: &mut Transform, anchor: Anchor, delta: Seconds) {
				unsafe {
					CALL_ARGS.push((*agent, anchor, delta));
				}
			}
		}

		let mut app = setup::<MyEvent>();
		app.world_mut().spawn(Camera3dBundle {
			transform: Transform::from_xyz(1., 2., 3.),
			..default()
		});
		app.world_mut().spawn(Camera3dBundle {
			transform: Transform::from_xyz(4., 5., 6.),
			..default()
		});

		tick_time(&mut app, Duration::from_secs(11));
		app.world_mut().send_event(MyEvent);
		app.world_mut().run_system_once(cam_movement::<MyEvent>);

		assert_eq!(
			vec![
				(
					Transform::from_xyz(1., 2., 3.),
					Anchor(Vec3::new(0., 0.5, 0.)),
					Seconds(11.)
				),
				(
					Transform::from_xyz(4., 5., 6.),
					Anchor(Vec3::new(0., 0.5, 0.)),
					Seconds(11.)
				)
			],
			unsafe { CALL_ARGS.clone() }
		)
	}
}
