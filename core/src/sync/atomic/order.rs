/*
    Appellation: order <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::sync::atomic::Ordering;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumCount, EnumIs, EnumIter, EnumString, VariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Display,
    EnumCount,
    EnumIs,
    EnumIter,
    EnumString,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize,),
    serde(rename_all = "snake_case", untagged)
)]
#[strum(serialize_all = "snake_case")]
pub enum AtomicOrder {
    Acquire,
    AcqRel,
    #[default]
    Relaxed,
    Release,
    SeqCst,
}

impl From<AtomicOrder> for Ordering {
    fn from(order: AtomicOrder) -> Self {
        match order {
            AtomicOrder::Acquire => Ordering::Acquire,
            AtomicOrder::AcqRel => Ordering::AcqRel,
            AtomicOrder::Relaxed => Ordering::Relaxed,
            AtomicOrder::Release => Ordering::Release,
            AtomicOrder::SeqCst => Ordering::SeqCst,
        }
    }
}

impl From<Ordering> for AtomicOrder {
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Acquire => AtomicOrder::Acquire,
            Ordering::AcqRel => AtomicOrder::AcqRel,
            Ordering::Release => AtomicOrder::Release,
            Ordering::SeqCst => AtomicOrder::SeqCst,
            _ => AtomicOrder::Relaxed,
        }
    }
}
