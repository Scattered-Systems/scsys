/*
    Appellation: gsw <module>
    Contrib: @FL03
*/
/// generates methods for some type or type reference
///
/// # Examples
///
/// ```rust
/// use scsys_core::gsw;
///
/// #[derive(Debug, Clone)]
///
/// struct MyStruct<T> {
///    a: u32,
///    b: u32,
///    c: T,
/// }
///
/// impl<T> MyStruct<T> {
///     gsw! {
///         a: u32,
///         b: u32,
///     }
///
///    gsw! {
///        c: &T,
///    }
/// }
/// ```
#[macro_export]
macro_rules! gsw {
    ($($name:ident: &$T:ty),* $(,)?) => {
        gsw!(@set $($name: $T),*);
        $(gsw!(@get $name: &$T);)*
    };
    ($($name:ident: $T:ty),* $(,)?) => {
        gsw!(@set $($name: $T),*);
        $(gsw!(@get $name: $T);)*
    };
    (@get $name:ident: &$T:ty) => {

        paste::paste! {
            pub const fn $name(&self) -> &$T {
                &self.$name
            }

            #[doc(inline)]
            pub fn [<$name _mut>] (&mut self) -> &mut $T {
                &mut self.$name
            }
        }
    };
    (@get $name:ident: $(&)?$T:ty) => {
        paste::paste! {
            pub const fn $name(&self) -> $T {
                self.$name
            }

            #[doc(inline)]
            pub fn [<$name _mut>] (&mut self) -> &mut $T {
                &mut self.$name
            }
        }
    };
    (@set $($name:ident: $T:ty),*) => {
        $(
            gsw!(@set_impl $name: $T);
        )*
    };
    (@set_impl $name:ident: $T:ty) => {
        paste::item! {
            pub fn [<set_ $name>](&mut self, $name: $T) -> &mut Self {
                *self.[<$name _mut>]() = $name;
                self
            }

            pub fn [<with_ $name>] (self, $name: $T) -> Self {
                Self {
                    $name,
                    ..self
                }
            }
        }
    };
}
