#[macro_export]
macro_rules! push_optional_filter {
    ($builder:expr, $field_name:expr, $check:expr) => {
        push_optional_filter!($builder, $field_name, $check, "");
    };
    ($builder:expr, $field_name:expr, $check_pre:expr, $check_post:expr) => {
        if let Some(val) = $field_name {
            $builder.push($check_pre);
            $builder.push_bind(val);
            $builder.push($check_post);
        }
    };
}
