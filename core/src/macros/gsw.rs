/*
    Appellation: gsw <module>
    Contrib: @FL03
*/

/// The `gsw` macro generates getter and setter methods for the fields of a struct. At the 
/// moment, the macro can handle any type; for types that implement the [`Copy`] trait, simply 
/// drop the `&` to the left of each type. 
/// 
/// **Note**: make sure that 
///
/// ### Usage
///
/// ```rust
/// use scsys_core::gsw;
///
/// #[derive(Clone, Debug, Default)]
/// pub struct Sample<T> {
///     pub(crate) a: usize,
///     pub(crate) b: f32,
///     pub(crate) store: Vec<u8>,
///     pub(crate) c:T,
/// 
/// }
///
/// impl<T> Sample<T> {
///     gsw! {
///         a: usize,
///         b: f32,
///     }
///     gsw! {
///         c: &T,
///         store: &Vec<u8>,
///     }
/// }
/// 
/// #[test]
/// fn test_sample_gsw_impls() {
///     let mut sample = Sample::<&str>::default().with_a(10).with_store(vec![1, 2, 3]);
///     sample.set_b(3.14).set_c("hello");
///     assert_eq!(sample.a(), 10);
///     assert_eq!(sample.b(), 3.14);
///     assert_eq!(sample.c(), "hello");
///     assert_eq!(sample.store(), &vec![1, 2, 3]);
///     sample.c_mut().push(u8::MAX);
///     assert!(sample.store().last() == Some(&u8::MAX));
/// }
/// ```
#[macro_export]
macro_rules! gsw {
    ($($name:ident: &$T:ty),* $(,)?) => {
        $(
            $crate::gsw!(@get $name: &$T);
            $crate::gsw!(@get_mut $name: $T);
            $crate::gsw!(@setter $name: $T);
        )*
    };
    ($($name:ident: $T:ty),* $(,)?) => {
        $(
            $crate::gsw!(@get $name: $T);
            $crate::gsw!(@get_mut $name: $T);
            $crate::gsw!(@setter $name: $T);
        )*
    };
    (@setter $name:ident: $T:ty) => {
        $crate::gsw!(@set $name: $T);
        $crate::gsw!(@with $name: $T);
    };
    (@get $name:ident: &$T:ty) => {
        pub const fn $name(&self) -> &$T {
            &self.$name
        }
    };
    (@get $name:ident: $T:ty) => {
        pub const fn $name(&self) -> $T {
            self.$name
        }
    };
    (@get_mut $name:ident: $T:ty) => {
        paste::paste! {
            pub const fn [<$name _mut>] (&mut self) -> &mut $T {
                &mut self.$name
            }
        }
    };
    (@set $name:ident: $T:ty) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, $name: $T) -> &mut Self {
                self.$name = $name;
                self
            }
        }
    };
    (@with $name:ident: $T:ty) => {
        paste::item! {
            pub fn [<with_ $name>] (self, $name: $T) -> Self {
                Self {
                    $name,
                    ..self
                }
            }
        }
    };
}
