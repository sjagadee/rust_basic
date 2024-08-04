#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}

fn create_car_color_blue() -> CarColor {
    let my_car_color: CarColor = CarColor::Blue;
    my_car_color
}

fn check_under_five(num_check:u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not Under 5!".to_string())
    }
}

fn check_under_five_built_in(num_check:u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not Under 5!".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enums() {
        let car_color: CarColor = create_car_color_blue();
        dbg!(car_color);

        let is_under_five_res: GivenResult<u8, String> = check_under_five(3);
        dbg!(is_under_five_res);

        let is_under_five_res: GivenResult<u8, String> = check_under_five(33);
        dbg!(is_under_five_res);

        let is_under_five_res_built_in: Result<u8, String> = check_under_five_built_in(3);
        dbg!(is_under_five_res_built_in);

        let is_under_five_res_built_in: Result<u8, String> = check_under_five_built_in(33);
        dbg!(is_under_five_res_built_in);

        let check_remainder: GivenOption<f32> = remainder_zero(12.2);
        dbg!(check_remainder);

        let check_remainder: GivenOption<f32> = remainder_zero(10.0);
        dbg!(check_remainder);

        let check_remainder_built_in: Option<f32> = remainder_zero_built_in(12.2);
        dbg!(check_remainder_built_in);

        let check_remainder_built_in: Option<f32> = remainder_zero_built_in(10.0);
        dbg!(check_remainder_built_in);
    }
}