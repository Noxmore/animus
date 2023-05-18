#![doc = include_str!("../README.md")]

pub mod prelude;
pub mod animators;

pub use lerp;

use std::{collections::{HashMap, hash_map::DefaultHasher}, time::Instant, ops, hash::{Hash, Hasher}};
use keystone::*;
use lerp::*;

// Yes i know i use tabs && some newline braces for big blocks, i come from c# land and this MY LIBRARY so i can do what i want :)

/// The hub of Animus, stores animations, and allows you to access/create them with it's `anim` method.
#[derive(Clone, Debug, Default)]
pub struct Animus {
	/// All animations currently in motion, storage format is: <animation id, (currently animating, for gc(), the [Instant] animation was created)>
	pub animations: HashMap<u64, (bool, Instant)>,
}

impl Animus
{
	// TODO: Create an anim function that changes an &mut f32, instead of having to call this every frame.

	/// Gets output value for a specific animation, if it does not exist, or is finished animating, create a new one.
	/// 
	/// `duration` is measured in seconds.
	/// 
	/// Note: it's not guaranteed that this will output 0.0 the first time it's run.
	/// 
	/// # Examples
	/// ```
	/// use animus::prelude::*;
	/// 
	/// let mut animus = Animus::default();
	/// 
	/// // It starts out at around 0.0, but when it gets called later, it will be a higher number as the animation progresses.
	/// let animation_value = animus.anim("test_anim", -50., 50., 3., linear);
	/// ```
	pub fn anim(&mut self, id: impl Hash, start: f32, end: f32, duration: f32, animator: impl Animator) -> f32
	{
		// Get the id of the animation
		let mut hasher = DefaultHasher::new();
		id.hash(&mut hasher);
		let id = hasher.finish();

		// If the animation does not already exist, create it.
		if !self.animations.contains_key(&id) {
			self.animations.insert(id, (true, Instant::now()));
		}

		let (animating, start_time) = self.animations.get_mut(&id).unwrap();

		let fraction = start_time.elapsed().as_secs_f32() / duration; // Calculate the required fraction
		if fraction <= 0. || fraction >= 1. { *animating = false } // If the animation has finished, mark it has not animating.
		start.lerp_bounded(end, animator(fraction))
	}

	/// Call at the end of every update/frame, removes animations that are not in use.
	pub fn gc(&mut self) {
		self.animations = self.animations.clone().into_iter().filter(|(_id, (animating, _anim))| *animating).collect();
	}
}


/// Animation function, takes in a fraction, and outputs another fraction, see [animators] for examples.
pub trait Animator: FnOnce(f32) -> f32 {}
impl<T: FnOnce(f32) -> f32> Animator for T {}

/// Currently not used, in the future i would like to use more then just f32s, but i'm not totally sure how yet.
trait Animatable: Sized + Lerp<f32> + ops::Mul + ops::MulAssign + ops::Add + ops::AddAssign {}
impl<T: Sized + Lerp<f32> + ops::Mul + ops::MulAssign + ops::Add + ops::AddAssign> Animatable for T {}