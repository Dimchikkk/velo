macro_rules! pair_struct {
    ($obj:ident . $field:ident) => {{
        let field_val = $obj.$field;
        (stringify!($field).to_string(), field_val)
    }};
}
