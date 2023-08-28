macro_rules! push_option_array {
    ($path:expr, $self:ident, $property:ident) => {
        if let Some(option) = &$self.$property {
            if !option.is_empty() {
                option.iter().enumerate().for_each(|(i, item)| {
                    $path.push_str("/");
                    $path.push_str(stringify!($property));
                    $path.push_str("[");
                    $path.push_str(&i.to_string());
                    $path.push_str("]/");
                    $path.push_str(&item.to_string());
                })
            }
        }
    };
}
pub(crate) use push_option_array;

macro_rules! push_option {
    ($path:expr, $self:ident, $property:ident) => {
        if let Some(option) = &$self.$property {
            $path.push_str("/");
            $path.push_str(stringify!($property));
            $path.push_str("/");
            $path.push_str(&option.to_string());
        }
    };
}
pub(crate) use push_option;

macro_rules! push_option_bool {
    ($path:expr, $self:ident, $property:ident) => {
        if let Some(option) = &$self.$property {
            if (*option) {
                $path.push_str("/");
                $path.push_str(stringify!($property));
                $path.push_str("/1");
            }
        }
    };
}
pub(crate) use push_option_bool;

macro_rules! push {
    ($path:expr, $self:ident, $property:ident) => {
        $path.push_str("/");
        $path.push_str(stringify!($property));
        $path.push_str("/");
        $path.push_str(&$self.$property.to_string());
    };
}
pub(crate) use push;
