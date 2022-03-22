use super::structs::{self, Collection};

pub trait DocumentContainer {
    fn get_document(&self, name: String) -> structs::Document where Self: Sized {
        let ret: structs::Document = structs::Document {
            parent: self,
            document_name: name,
        };
        return ret;
    }
    fn get_url(&self) -> String;
}

impl DocumentContainer for structs::Collection<'_> {
    fn get_url(&self) -> String {
        return format!("{}{}", self.parent.get_url(), self.collection_name);
    }
}