//! All built-in animation functions, and animators, some are straight animation functions (Fn(f32) -> f32),
//! while others are animators (Fn(...) -> impl Animator), which produce animation functions.

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



/// Combines 2 animators, `start` occurring on the starting half of the animation, and `end` occurring on the second half of the animation.
/// 
/// Useful for ease-in-ease-out animations, but it's recommended to use [sin_sigmoid] or [ease_in_out] if possible for more concise code.
pub fn in_out(start: impl Animator, end: impl Animator) -> impl Animator {
	move |f|
	if f < 0.5
		{ start(f * 2.) / 2. }
		else { end((f - 0.5) * 2.) / 2. + 0.5 }
}


/// Invokes the specified [Animator] in half the time, and for the other half, run the same animator, but flipped, creating a spike-like curve.
pub fn spike(anim: impl Animator) -> impl Animator {
	move |f|
	if f <= 0.5
		{ anim(f / 0.5) }
		else { anim((1. - f) / 0.5) }
}
/// Like [spike], but stays at the end position for a certain amount of time before returning.
/// 
/// `time`: A range from `0..1` representing the percentage of the time should be idle (e.g. 0.5 = half the time), increasing this also makes the transition durations faster.
pub fn spike_stay(time: f32, anim: impl Animator) -> impl Animator {
	let transition_time = 0.5 - time / 2.;
	move |f|
	if f <= transition_time
		{ anim(f / transition_time) }
	else if f >= 1. - transition_time
		{ anim((1. - f) / transition_time) }
	else { 1. }
}