use crate::prelude::*;

pub struct LinearAnimator;

impl Animator for LinearAnimator {
	fn update(dt: f32, value: f32, target: f32, duration: f32) -> f32 {
		value.lerp(target, t)
	}
}