/*
    Appellation: appellation <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        An appellation is defined to be a name or title given to a person or thing.
        In this context, an appellation is a name or title given to a computational object.
        Appellation's are mathematically similar to a basis from linear-algebra; allowing all variations of the objects to be
        simply represented via a single identifier. This is critical for the efficient storage and retrieval of data from a multi-layered
        peer-to-peer network. For instance, when a user requests a specific piece of data, the request is first processed locally, searching
        for the data on the users personal network. If the data is not found, the request is then forwarded to the public layer of the network before
        finally skimming a type of data-marketplace for the data.

        An appellation can best best be thought of as a three-tuple of elements: (id, name, title).
*/

pub trait Appellation<I, J, K> {
    fn id(&self) -> &I;
    fn name(&self) -> &J;
    fn title(&self) -> &K;
}
