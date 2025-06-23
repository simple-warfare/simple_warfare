#[macro_export]
macro_rules! add_field_method_fields {
    ($fields:ident{ $($field:ident),* $(,)? }) => {
        $($fields.add_field_method_get(stringify!($field), |_, this| Ok(this.$field.clone()));
        $fields.add_field_method_set(stringify!($field), |_, this, val| {this.$field = val;Ok(())});)*
    };
}

#[macro_export]
macro_rules! add_field_function_fields {
    ($fields:ident{ $($field:ident),* $(,)? }) => {
        $($fields.add_field_function_get(stringify!(u$field), |_, ud| ud.user_value::<Option<String>>());
        $fields.add_field_function_set(stringify!(u$field), |_, ud, s: Option<String>| ud.set_user_value(s));)*
    };
}
