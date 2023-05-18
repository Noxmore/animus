use crate::prelude::*;

/// The simplest animation function, goes in a straight line.
pub fn linear(f: f32) -> f32 { f }

/// Ease in animator, the lower the weight is, the closer to a straight line the animation will be, a good starting weight is 3.
pub fn ease_in(weight: f32) -> impl Animator { move |f| f.powf(weight) }
/// Ease out animator, the lower the weight is, the closer to a straight line the animation will be, a good starting weight is 3.
pub fn ease_out(weight: f32) -> impl Animator { move |f| 1. - (1. - f).powf(weight) }
/// Shorthand for using [in_out] with [ease_in], and [ease_out] both using the same weight.
pub fn ease_in_out(weight: f32) -> impl Animator { in_out(ease_in(weight), ease_out(weight)) }

/// Transforms `f` to a sigmoid value using sin function in the range `0.0..1.0`. Produces an ease-in-ease-out result.
pub fn sin_sigmoid(f: f32) -> f32 { (( (f - 0.5) * std::f32::consts::PI).sin() + 1. ) / 2. }



/// Combines 2 animators, `start` occuring on the starting half of the animation, and `end` occuring on the second half of the animation.
/// 
/// Useful for ease-in-ease-out animations, but it's recommended to use [sin_sigmoid] or [ease_in_out] if possible for more concise code.
pub fn in_out(start: impl Animator, end: impl Animator) -> impl Animator {
	move |f|
	tr!(f < 0.5 =>
		start(f * 2.) / 2.
		; end((f - 0.5) * 2.) / 2. + 0.5
	)
}


/// Invokes the specified [Animator] in half the time, and for the other half, run the same animator, but flipped, creating a spike-like curve.
pub fn spike(anim: impl Animator) -> impl Animator {
	move |f|
	tr!(f <= 0.5 =>
		anim(f / 0.5)
		; anim((1. - f) / 0.5)
	)
}
/// Like [spike], but stays at the end position for a certain amount of time before returning.
/// 
/// `time`: A range from `0..1` representing the percentage of the time should be idle (e.g. 0.5 = half the time), increasing this also makes the transition durations faster.
pub fn spike_stay(time: f32, anim: impl Animator) -> impl Animator {
	let transition_time = 0.5 - time / 2.;
	move |f|
	tr!(f <= transition_time =>
			anim(f / transition_time)
		, f >= 1. - transition_time =>
			anim((1. - f) / transition_time)
		; 1.
	)
}

/*
/// Lerps between 2 animators.
pub struct InOut<In: Animator, Out: Animator> {
	in_anim: In,
	out_anim: Out,
}
impl<In: Animator, Out: Animator> Animator for InOut<In, Out> {
	fn get(&mut self, fraction: f32) -> f32 {
		self.in_anim.get(fraction).lerp(self.out_anim.get(fraction), fraction)
	}
}
impl<In: Animator, Out: Animator> InOut<In, Out> {
	/// Creates a new [Both] with the specified animators.
	pub fn new(in_anim: In, out_anim: Out) -> Self { Self { in_anim, out_anim } }
}

/// TODO: Document
pub struct Spike<Anim: Animator> {
	anim: Anim,
}
impl<Anim: Animator> Animator for Spike<Anim> {
	fn get(&mut self, fraction: f32) -> f32
	{
		tr!(fraction <= 0.5 =>
			self.anim.get(fraction / 0.5)
			; self.anim.get((1. - fraction) / 0.5)
		)
	}
}
impl<Anim: Animator> Spike<Anim> {
	/// Creates a new [Spike] with the specified [Animator]
	pub fn new(anim: Anim) -> Self { Self { anim } }

	pub fn stay(anim: Anim, time: f32) -> Self { todo!() }
}

/// The simplest animator, goes in a streight line.
pub struct Linear;
impl Animator for Linear {
	fn get(&mut self, fraction: f32) -> f32 {
		fraction
	}
}

/// Smoothly eases in and out TODO: Document
pub struct Ease {
	weight: i32,
	flip: bool,
}
impl Animator for Ease {
	fn get(&mut self, fraction: f32) -> f32
	{
		tr!(self.flip =>
			1. - (1. - fraction).powi(self.weight)
			; fraction.powi(self.weight)
		)
	}
}
impl Ease {
	/// Ease in function, `in` is a rust keyword unfortunately, so it has to be `in_`
	pub fn in_(weight: i32) -> Self {
		Self {
			weight,
			flip: false,
		}
	}

	/// Ease out function
	pub fn out(weight: i32) -> Self {
		Self { weight, flip: true }
	}

	/// Create a [InOut] with a [Ease] in and [Ease] out, with with the same weight
	pub fn in_out(weight: i32) -> InOut<Self, Self> {
		InOut::new(Self::in_(weight), Self::out(weight))
	}
}
*/