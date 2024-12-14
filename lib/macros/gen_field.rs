#[macro_export]
macro_rules! gen_field {
    ($($v:tt)*) => {
        {
            use quote::quote;
            use syn::{parse::Parser, Field};
            Parser::parse2(Field::parse_named, quote!($($v)*)).unwrap()
        }
    };
}
