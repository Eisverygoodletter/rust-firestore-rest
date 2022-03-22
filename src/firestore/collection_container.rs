use super::structs::{self, Collection};
pub trait CollectionContainer {
    fn get_collection(&self, name: String) -> structs::Collection where Self: Sized {
        let ret: structs::Collection = structs::Collection {
            parent: self,
            collection_name: name,
        };
        return ret;
    }
    fn get_url(&self) -> String;
}

impl CollectionContainer for structs::Firestore {
    fn get_url(&self) -> String {
        return self.db_url.clone();
    }
}

impl CollectionContainer for structs::Document<'_> {
    fn get_url(&self) -> String {
        return format!("{}{}", self.parent.get_url(), self.document_name);
    }
}