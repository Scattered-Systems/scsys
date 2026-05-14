/*
    appellation: impl_id_rand <module>
    authors: @FL03
*/
use crate::id::Id;
use rand::{Rng, RngExt};
use rand_distr::uniform::{SampleRange, SampleUniform};
use rand_distr::{Distribution, StandardNormal, StandardUniform};

impl<T> Id<T> {
    /// generate a new, random identifier from a value of type `T`
    pub fn random() -> Self
    where
        StandardUniform: Distribution<T>,
    {
        Self::random_with(&mut rand::rng(), StandardUniform)
    }

    pub fn random_between<R>(range: R) -> Self
    where
        R: SampleRange<T>,
        T: SampleUniform,
    {
        Self::new(rand::random_range(range))
    }
    /// generate a random index from a value of type `T` using the provided [`Rng`](rand::Rng)
    pub fn random_with<R, Dist>(rng: &mut R, distr: Dist) -> Self
    where
        R: ?Sized + Rng,
        Dist: Distribution<T>,
    {
        // generate a random u128 and cast it to usize
        let rid = rng.sample(distr);
        // cast the random u128 to usize
        Self::new(rid)
    }
}

impl<T> Distribution<Id<T>> for StandardNormal
where
    StandardNormal: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Id<T> {
        Id::new(rng.sample(StandardNormal))
    }
}

impl<T> Distribution<Id<T>> for StandardUniform
where
    StandardUniform: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Id<T> {
        Id::new(rng.sample(StandardUniform))
    }
}
