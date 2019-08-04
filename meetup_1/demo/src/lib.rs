use snafu::Snafu;

#[derive(Debug, Snafu, Clone)]
pub enum MyError {
    #[snafu(display("MyError is ErrA: {}", msg))]
    ErrA { msg: String },
    #[snafu(display("MyError is ErrB"))]
    ErrB,
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