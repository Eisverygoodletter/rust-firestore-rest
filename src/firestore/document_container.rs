use super::structs::{Collection, Document};
use super::raw_data::RawDataContainer;
pub trait DocumentContainer: RawDataContainer + Sync + Send {
    fn get_document(&self, name: String) -> Document where Self: Sized {
        let ret: Document = Document {
            parent: self,
            document_name: name,
        };
        return ret;
    }
}

impl DocumentContainer for Collection<'_> {}