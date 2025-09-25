use num_traits::float::FloatCore;

pub fn percent_of(number: u32, percent: u32) -> u32 {
    let result = (number as f64) * (percent as f64) / 100.0;
    result.round() as u32
}

pub fn percent_of_to_nearest_odd(number: u32, percent: u32) -> u32 {
    let result = (number as f64) * (percent as f64) / 100.0;
    let rounded = result as u32;

    if rounded % 2 == 1 {
        rounded
    } else if rounded == 0 {
        1
    } else {
        let dist_down = (result - (rounded - 1) as f64).abs();
        let dist_up = (result - (rounded + 1) as f64).abs();

        if dist_down <= dist_up {
            rounded - 1
        } else {
            rounded + 1
        }
    }
}
