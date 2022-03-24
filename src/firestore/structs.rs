use super::{document_container, collection_container};
use reqwest;

/// Representation of a Firestore database REST origin
/// see `FirestoreBuilder` for more details
pub struct Firestore {
    pub db_url: String,
    pub(crate) reqwest_client: reqwest::Client,
}

pub struct FirestoreBuilder {
    pub(crate) db_url: String,
}

pub struct Collection<'collection_lifetime> {
    // the parent must live as long as the collection
    pub parent: &'collection_lifetime dyn collection_container::CollectionContainer,
    pub collection_name: String,
}

pub struct Document<'document_lifetime> {
    // the parent must live as long as the collection
    pub parent: &'document_lifetime dyn document_container::DocumentContainer,
    pub document_name: String,
}