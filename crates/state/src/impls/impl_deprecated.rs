/*
    Appellation: impl_deprecated <module>
    Created At: 2025.09.08:19:28:18
    Contrib: @FL03
*/
use crate::state::State;

use crate::traits::RawState;

#[doc(hidden)]
impl<Q> State<Q>
where
    Q: RawState,
{
    #[deprecated(
        since = "0.2.9",
        note = "use `value` instead, this will be removed in the next major release"
    )]
    pub fn into_inner(self) -> Q {
        self.0
    }
}
