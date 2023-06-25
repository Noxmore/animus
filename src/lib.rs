#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

pub mod prelude;
pub mod animators;

pub use lerp;

use std::{collections::{HashMap, hash_map::DefaultHasher}, time::Instant, hash::{Hash, Hasher}};
use lerp::*;

// Yes i know i use tabs && some newline braces for big blocks, i come from c# land and this MY LIBRARY so i can do what i want :)

/// The hub of Animus, stores animations, and allows you to access/create them with it's `anim` method.
#[derive(Clone, Debug, Default)]
pub struct Animus {
	/// All animations currently in motion, storage format is: <animation id, (currently animating (for gc()), the [Instant] animation was created)>
	pub animations: HashMap<u64, CachedAnimation>,
}

impl Animus
{
	// TODO: Create an anim function that changes an &mut f32, instead of having to call this every frame.

	/// Gets output value for a specific animation, if it does not exist, or is finished animating, create a new one.
	/// 
	/// `duration` is measured in seconds.
	/// 
	/// Note: it's not guaranteed that this will output the value of `start` the first time it's run.
	/// 
	/// # Examples
	/// ```
	/// use animus::prelude::*;
	/// 
	/// let mut animus = Animus::default();
	/// 
	/// // It starts out at around -50., but when it gets called later, it will be a higher number as the animation progresses.
	/// let animation_value = animus.anim("test_anim", -50., 50., 3., linear);
	/// ```
	pub fn anim<T: Animatable>(&mut self, id: impl Hash, start: T, end: T, duration: f32, animator: impl Animator) -> T
	{
		// Get the id of the animation
		let mut hasher = DefaultHasher::new();
		id.hash(&mut hasher);
		let id = hasher.finish();

		// If the animation does not already exist, create it.
		let anim = self.animations.entry(id).or_insert(CachedAnimation::new());

		let fraction = anim.start_time.elapsed().as_secs_f32() / duration; // Calculate the required fraction
		if fraction <= 0. || fraction >= 1. { anim.animating = false } // If the animation has finished, mark it has not animating.

		start.lerp_bounded(end, animator(fraction))
	}

	// TODO: I would rather use `Drop` to do this, but at the moment it would just as, or more cumbersome

	/// Animation garbage collector, call at the end of every update/frame, removes animations that are not in use.
	pub fn gc(&mut self) {
		self.animations = self.animations.clone().into_iter().filter(|(_id, anim)| anim.animating).collect();
	}
}


/// An animation currently animating
#[derive(Clone, Debug)]
pub struct CachedAnimation
{
	/// Whether or not this animation is currently animating, if false, it will get removed by the garbage collector
	pub animating: bool,
	/// The time the animation started
	pub start_time: Instant,
}

impl CachedAnimation
{
	/// Creates a new `CachedAnimation` that is currently animating, and has it's start_time as `Instant::now()`
	pub fn new() -> Self {
		Self {
			animating: true,
			start_time: Instant::now(),
		}
	}
}


/// Animation function, takes in a fraction, and outputs another fraction, see [animators] for examples.
pub trait Animator: FnOnce(f32) -> f32 {}
impl<T: FnOnce(f32) -> f32> Animator for T {}

/// An animatable value, automatically implemented for any type that satisfies it's trait requirement
pub trait Animatable: Sized + Lerp<f32> {}
impl<T: Sized + Lerp<f32>> Animatable for T {}