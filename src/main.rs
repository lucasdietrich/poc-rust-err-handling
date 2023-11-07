// import our crate
use myerr::MyErrorTrait;

mod utils;
use utils::*;

#[derive(Debug, MyErrorTrait)]
#[repr(C)]
pub enum MySpecificError {
    #[success]
    Ok,
    Einval,
    Enomem,
    Custom1,
    Custom2,
}

// This code is equivalent to:
//
// #[derive(Debug)]
// #[repr(C)]
// pub enum MySpecificError {
//     Ok,
//     Einval,
//     Enomem,
//     Custom1,
//     Custom2,
// }
//
// impl<T> From<Result<T, MySpecificError>> for MySpecificError {
//     fn from(ret: Result<T, MySpecificError>) -> Self {
//         match ret {
//             Ok(_) => MySpecificError::Ok,
//             Err(err) => err,
//         }
//     }
// }

fn run_inner(val: u32) -> Result<(), MySpecificError> {
    let val = guard_valid_or(val, MySpecificError::Einval)?;
    let val = guard_valid2_or(val, MySpecificError::Enomem)?;

    if val == 2 {
        return Err(MySpecificError::Custom1);
    } else {
        Ok(())
    }
}

pub unsafe extern "C" fn run(val: u32) -> MySpecificError {
    run_inner(val).into()
}

fn main() {
    let err = unsafe { run(10) };
    println!("err: {:?}", err);
}
