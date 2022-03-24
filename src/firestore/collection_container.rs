use super::structs::{Document, Firestore, Collection};
use super::raw_data::RawDataContainer;
pub trait CollectionContainer : RawDataContainer + Sync + Send {
    fn get_collection(&self, name: String) -> Collection where Self: Sized {
        let ret: Collection = Collection {
            parent: self,
            collection_name: name,
        };
        return ret;
    }
}

impl CollectionContainer for Firestore {}

impl CollectionContainer for Document<'_> {}