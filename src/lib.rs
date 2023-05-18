#![doc = include_str!("../README.md")]

pub use glam;
pub mod prelude;
pub mod animators;

use std::{collections::{HashMap, hash_map::DefaultHasher}, time::Instant, ops, hash::{Hash, Hasher}};
use keystone::*;
use prelude::*;


#[derive(Clone, Debug, Default)]
pub struct Animus
{
	pub animations: HashMap<u64, (bool, Animation)>,
}

impl Animus
{
	/*fn new_anim(&mut self, id: u64, start: f32, end: f32, duration: f32) -> Animation {
		let anim = Animation::new(start, end, duration);
		self.animations.insert(id, (true, anim));
		anim
	}*/

	pub fn anim(&mut self, id: impl Hash, start: f32, end: f32, duration: f32, animator: impl Animator) -> f32
	{
		let mut hasher = DefaultHasher::new();
		id.hash(&mut hasher);
		let id = hasher.finish();

		/*let anim =
			if let Some((_animating, anim)) = self.animations.get_mut(&id) {
				if anim.start != start || anim.end != end || anim.duration != duration {
					&mut self.new_anim(id, start, end, duration)
				}
				else { anim }
			}
			else { &mut self.new_anim(id, start, end, duration) };*/

		if !self.animations.contains_key(&id) {
			self.animations.insert(id, (true, Animation::new(start, end, duration)));
		}

		let (animating, anim) = self.animations.get_mut(&id).unwrap();

		let fraction = anim.start_time.elapsed().as_secs_f32() / anim.duration;
		if fraction <= 0. || fraction >= 1. { *animating = false }
		anim.start.lerp_bounded(anim.end, animator(fraction))
	}

	pub fn is_animating(&self) -> bool {
		!self.animations.is_empty()
		// TODO: Check is animations are done
	}

	pub fn clear_animations(&mut self) {
		self.animations.clear();
	}


	/// Call at the end of every update/frame, removes animations that are not in use.
	pub fn gc(&mut self)
	{
		// println!("before: {:#?}", self.animations);
		self.animations = self.animations.clone().into_iter().filter(|(_id, (animating, _anim))| *animating).collect();
		// println!("after: {:#?}", self.animations);
	}
}

#[derive(Clone, Copy, Debug)]
pub struct Animation
{
	pub start: f32,
	pub end: f32,

	pub start_time: Instant,
	pub duration: f32,
	// animator: Box<dyn Animator>,
}

impl Animation
{
	pub fn new(start: f32, end: f32, duration: f32) -> Self {
		Self {start, end, start_time: Instant::now(), duration}
	}
}


/// Currently not used
pub trait Animatable: Sized + Lerp<f32> + ops::Mul + ops::MulAssign + ops::Add + ops::AddAssign {}
impl<T: Sized + Lerp<f32> + ops::Mul + ops::MulAssign + ops::Add + ops::AddAssign> Animatable for T {}




/*pub trait Animator
{
	fn get_from_animation(&mut self, anim: Animation, fraction: f32) -> f32 {
		anim.start.lerp_bounded(anim.end, self.get(fraction))
	}

	fn get(&mut self, fraction: f32) -> f32;
}*/

pub trait Animator: FnOnce(f32) -> f32 {}
impl<T: FnOnce(f32) -> f32> Animator for T {}

// pub type Animator = FnOnce(f32) -> f32;