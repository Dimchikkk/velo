#[macro_export]
macro_rules! struct_pair {
    ($obj:expr . $field:ident) => {{
        let field_val = $obj.$field;
        (stringify!($field), field_val)
    }};
}
