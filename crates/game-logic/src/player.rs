use rand::Rng;
use rand::rngs::SmallRng;

use pico_display::aliases::Display;
use pico_display::dice::Dice;

pub fn roll_die<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    let dice = Dice::roll(face_value, 1);
    dice.draw(target).map(|_| small_rng)
}

pub fn roll_two_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    let dice = Dice::roll(face_value, 2);
    dice.draw(target).map(|_| small_rng)
}

pub fn roll_three_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    let dice = Dice::roll(face_value, 3);
    dice.draw(target).map(|_| small_rng)
}

pub fn roll_four_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    let dice = Dice::roll(face_value, 4);
    dice.draw(target).map(|_| small_rng)
}

pub fn roll_five_dice<T>(target: &mut T, mut small_rng: SmallRng) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    let face_value = || small_rng.random();
    let dice = Dice::roll(face_value, 5);
    dice.draw(target).map(|_| small_rng)
}

pub fn roll_one_to_five_number_of_dice<T>(
    target: &mut T,
    mut small_rng: SmallRng,
) -> Result<SmallRng, T::Error>
where
    T: Display,
{
    match small_rng.random_range(1..6) {
        1 => roll_die(target, small_rng),
        2 => roll_two_dice(target, small_rng),
        3 => roll_three_dice(target, small_rng),
        4 => roll_four_dice(target, small_rng),
        _ => roll_five_dice(target, small_rng),
    }
}
