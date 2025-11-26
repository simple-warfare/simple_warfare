pub mod unit;

#[macro_export]
macro_rules! define_components {
    (@tt $type:ty, ($name:ident, $($names:ident),+), $($metas:meta)*)=>{
        $(#[$metas])*
        pub struct $name(pub $type);

        define_components!{@tt $type, ($($names),+), $($metas)*}
    };
    (@tt $type:ty, ($name:ident), $($metas:meta)*)=>{
        $(#[$metas])*
        pub struct $name(pub $type);
    };
    ($(#[$metas:meta])* $($names:ident)|+($type:ty))=>{
        define_components!{@tt $type, ($($names),+), $($metas)*}
    };
}
