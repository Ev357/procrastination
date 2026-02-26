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
