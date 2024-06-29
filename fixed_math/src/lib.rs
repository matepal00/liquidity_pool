#[cfg(test)]
mod unit_tests;
fn multiplyer() -> u64 {
    100000
}
fn digits() -> u64 {
    (multiplyer().to_string().len() - 1) as u64
}
fn max_u64() -> u64 {
    u64::MAX
}
fn max_new_number() -> u64 {
    max_u64() / multiplyer()
}
fn max_number_to_add() -> u64 {
    u64::MAX / 2
}
fn max_number_to_multiply() -> u64 {
    4294967296
}
fn max_dividend() -> u64 {
    max_new_number()
}
fn max_divisor() -> u64 {
    multiplyer().pow(2) * 1000
}
fn reciprosal(x: u64) -> u64 {
    multiplyer().pow(2) * 1000 / x
}
fn integer(x: u64) -> u64 {
    (x / multiplyer()) * multiplyer()
}
fn fractional(x: u64) -> u64 {
    x - integer(x)
}
pub fn add(x: u64, y: u64) -> Result<u64, Errors> {
    if x > max_number_to_add() || y > max_number_to_add() {
        return Err(Errors::Overflow);
    }
    Ok(x + y)
}
pub fn multiply(x: u64, y: u64) -> Result<u64, Errors> {
    let max_number_to_multiply: u64 = max_number_to_multiply();
    if x > max_number_to_multiply * 1000 || y > max_number_to_multiply {
        return Err(Errors::Overflow);
    }
    let x_int: u64 = integer(x) / multiplyer();
    let x_frac: u64 = fractional(x);
    let y_int: u64 = integer(y) / multiplyer();
    let y_frac: u64 = fractional(y);
    let x_int_y_int: u64 = x_int * y_int;
    if x_int_y_int > max_new_number() {
        return Err(Errors::Overflow);
    }
    let x_int_y_frac: u64 = x_int * y_frac;
    let x_frac_y_int: u64 = x_frac * y_int;
    let x_frac_y_frac: u64 = (x_frac * y_frac) / multiplyer();
    Ok(add(
        add(x_int_y_int * multiplyer(), x_int_y_frac).unwrap(),
        add(x_frac_y_int, x_frac_y_frac).unwrap(),
    )
    .unwrap())
}
pub fn divide(x: u64, y: u64) -> Result<u64, Errors> {
    if x > max_dividend() || y > max_divisor() {
        return Err(Errors::Overflow);
    }
    match y {
        y if y == 0 => Err(Errors::DivideByZero),
        y if y == multiplyer() => Ok(x),
        y if x == y => Ok(multiplyer()),
        _ => Ok(multiply(x * 1000, reciprosal(y)).unwrap() / 1000000),
    }
}
pub fn f64_to_u64(f64: f64) -> Result<u64, Errors> {
    if f64 < 0.0 {
        return Err(Errors::NegativeNumber);
    }
    let multiplyer: u64 = multiplyer();
    let f64_to_string: String = f64.to_string();
    let parts: Vec<&str> = f64_to_string.split('.').collect();
    if parts[0].parse::<u64>().unwrap() > max_new_number() {
        return Err(Errors::NumberTooBig);
    }
    if parts.len() == 1 {
        Ok(parts[0].parse::<u64>().unwrap() * multiplyer)
    } else {
        let frac_part_len: usize = parts[1].len();
        let new_multiplyer: u64 = multiplyer / 10_u64.pow(frac_part_len as u32);
        Ok(parts.join("").parse::<u64>().unwrap() * new_multiplyer)
    }
}

pub fn u64_to_f64(u64: u64) -> f64 {
    let u64_to_string: String = u64.to_string();
    let moved_by_places_amount: usize = digits() as usize;
    let len: usize = u64_to_string.len();
    let float_str: String = if len <= moved_by_places_amount {
        let actual_len: usize = u64_to_string.len();
        let missing_places_amount: usize = moved_by_places_amount - actual_len;
        if missing_places_amount == 0 {
            format!("0.{}", u64_to_string)
        } else {
            let zeros: String = "0".repeat(missing_places_amount).to_string();
            let missing_part: String = zeros + &u64_to_string;
            format!("0.{}", missing_part)
        }
    } else {
        let int_part: &str = &u64_to_string[..len - moved_by_places_amount];
        let frac_part: &str = &u64_to_string[len - moved_by_places_amount..];
        format!("{}.{}", int_part, frac_part)
    };
    float_str.parse::<f64>().unwrap()
}

#[derive(Debug)]
pub enum Errors {
    Overflow,
    NegativeNumber,
    DivideByZero,
    NumberTooBig,
}
