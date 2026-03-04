#[macro_export]
macro_rules! matrix {
    ( $( $( $x:expr ),* $(,)? );* $(;)? ) => {
        crate::matrix::Matrix::new([
            $(
                [ $($x),* ],
            )*
        ])
    };
}

#[macro_export]
macro_rules! ss {
    ($x:expr) => {
        crate::super_string::SuperString::new($x)
    };
}
