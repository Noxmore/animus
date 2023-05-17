#![doc = include_str!("../README.md")]

pub use glam;
pub mod prelude;
pub mod animators;

use std::{collections::HashMap, time::Instant};
use prelude::*;


#[derive(Debug)]
pub struct Animus<'a>
{
	pub last_update: Instant,
	pub animations: HashMap<Uuid, Animation<'a>>,
}

impl<'a> Animus<'a>
{
	pub fn update(&mut self)
	{

	}

	pub fn animate_value(&mut self, id: impl Into<Uuid>, value: &mut f32, target: f32, duration: f32, animator: impl Animator) -> f32
	{

	}
}

#[derive(Debug)]
pub struct Animation<'a>
{
	value: &'a mut f32,
	duration: f32,
	target: f32,
	// animator: Box<dyn Animator>,
}


// pub trait Animatable: ;

pub trait Animator {
	fn update(dt: f32, value: f32, target: f32, duration: f32) -> f32;
}