pub fn guard_valid_or<T>(val: u32, err: T) -> Result<u32, T> {
    if val != 0 {
        Ok(val)
    } else {
        Err(err)
    }
}

pub fn guard_valid2_or<T>(val: u32, err: T) -> Result<u32, T> {
    if val != 1 {
        Ok(val)
    } else {
        Err(err)
    }
}
