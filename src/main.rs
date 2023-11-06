#[derive(Debug)]
#[repr(C)]
pub enum MySpecificError {
    Ok,
    Einval,
    Enomem,
    Custom1,
    Custom2,
}

impl CtypeError for MySpecificError {
    fn success_value() -> Self {
        MySpecificError::Ok
    }
}

impl From<MyGenericError> for MySpecificError {
    fn from(err: MyGenericError) -> Self {
        match err {
            MyGenericError::Ok => MySpecificError::Ok,
            MyGenericError::Einval => MySpecificError::Einval,
            MyGenericError::Enomem => MySpecificError::Enomem,
        }
    }
}

enum MyGenericError {
    Ok,
    Einval,
    Enomem,
}

trait CtypeError: From<MyGenericError> {
    fn success_value() -> Self;
}

fn is_valid(val: u32) -> Result<(), MyGenericError> {
    if val == 0 {
        Ok(())
    } else {
        Err(MyGenericError::Einval)
    }
}

fn run_inner(val: u32) -> Result<(), MyGenericError> {
    let z = is_valid(val)?;

    Ok(())
}

fn cerr<T>(ret: Result<(), MyGenericError>) -> T 
    where T: CtypeError
{
    match ret {
        Ok(_) => T::success_value(),
        Err(err) => err.into(),
    }
}

pub fn run(val: u32) -> MySpecificError {
    let ret = run_inner(val);

    match ret {
        Ok(_) => MySpecificError::Ok,
        Err(err) => err.into(),
    }
}

pub fn run2(val: u32) -> MySpecificError {
    let ret = run_inner(val);

    cerr(ret)
}

fn main() {
    let err = run(10);
    println!("err: {:?}", err);
}
