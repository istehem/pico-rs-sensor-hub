use rand::rngs::SmallRng;
use rand::Rng;

use crate::aliases::Display;
use crate::dice;

pub fn roll_die<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    dice::draw_dice(target, 1, face_value).map(|_| small_rng)
}

pub fn roll_two_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    dice::draw_dice(target, 2, face_value).map(|_| small_rng)
}

pub fn roll_three_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    dice::draw_dice(target, 3, face_value).map(|_| small_rng)
}

pub fn roll_four_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    dice::draw_dice(target, 4, face_value).map(|_| small_rng)
}

pub fn roll_five_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    dice::draw_dice(target, 5, face_value).map(|_| small_rng)
}
