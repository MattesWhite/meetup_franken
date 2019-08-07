use std::fs::File;
use snafu::{Snafu, ResultExt};

pub struct PubError(Error);

impl PubError {
    pub fn with_context(&self) -> bool {
        self.0.has_fields()
    }
}

impl From<Error> for PubError {
    fn from(ek: Error) -> Self {
        Self(ek)
    }
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Error is ErrA: {}", msg))]
    ErrA { msg: String },
    #[snafu(display("Error is ErrB"))]
    ErrB,
    #[snafu(display("Error is ErrC because of {}", source))]
    ErrC { source: std::io::Error },
}

impl Error {
    fn has_fields(&self) -> bool {
        match self {
            Error::ErrA {..} | Error::ErrC {..} => true,
            Error::ErrB => false,
        }
    }
}

#[allow(dead_code)]
fn err_a() -> Result<(), Error> {
    ErrA { msg: "My error msg".to_owned() }.fail()
}

#[allow(dead_code)]
fn err_b() -> Result<u32, Error> {
    use std::collections::HashMap;

    let map: HashMap<u32, u32> = HashMap::new();
    map.get(&42).cloned().ok_or_else(|| Error::ErrB)
}

#[allow(dead_code)]
fn err_c() -> Result<File, Error> {
    File::open("config.json").context(ErrC)
}

#[macro_export]
macro_rules! print_size {
    ($ty:ty) => {
        print_size!($ty, 30);
    };
    ($ty:ty, $width:expr) => {
        {
            let what = format!("Size of {}:", stringify!($ty));
            println!("{:<width$}{:3}", what, std::mem::size_of::<$ty>(), width = $width);
        }
    };
}