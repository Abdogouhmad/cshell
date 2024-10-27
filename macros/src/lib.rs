#[macro_export]
macro_rules! cmd {
    ($( $command:expr, $($args:expr),* );* ) => {
        $(
            println!("command: {}, args: {:?}", $command, vec![$($args),*]);
        )*
    };
}
