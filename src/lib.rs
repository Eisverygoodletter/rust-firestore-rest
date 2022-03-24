
//! A crate for interacting with the Firestore REST API
//! This crate is under construction and probably shouldn't be used
//! unless you want to bother looking at the source code.
//! 
//! example:
//! ```rust no_run
//! // include the firestore crate
//! use firestore::FirestoreBuilder;
//! // create a firestore builder
//! let firestoreBuilder = FirestoreBuilder::new();
//! // construct the firestore instance using the builder
//! let firestore = firestoreBuilder
//!                     .from_url("https://firestore.googleapis.com/v1/projects/my-very-cool-project-db/databases/(default)/".to_string())
//!                     .build();
//! 
//! // get a REPRESENTATION of the collection. This function does NOT guarantee that the collection exists, is valid,
//! // accesssible, etc. It is only a representation of the collection's path.
//! let users_collection = firestore.get_collection("users/");
//! 
//! // this function actually tries to get the complete json representation of the collection
//! let actual_json_value: Value = users_collection.get_complete_json().await.unwrap();
//! 
//! // find a document inside the users collection that corresponds to jimmy.
//! let jimmy_document = users_collection.get_document("jimmy/");
//! let jimmy_json_value: Value = jimmy_document.get_complete_json().await.unwrap();
//! 
//! // get the value of the "name" field of the jimmy document
//! let jimmy_name_value: Value = jimmy_json_value["fields"]["name"]["stringValue"].clone();
//! 
//! // get jimmy's actual name
//! let name_string: String = jimmy_name_value.as_str().unwrap();
//! ```
//! micro FAQ about the example above:
//! 1. wait... why is there a `stringValue`? This is because for some reason serde_json parses the strings that way. 
//!     I haven't gotten around to fixing it
//! 2. What is the purpose of the clone at the end? using the square bracket syntax makes serde_json return a &Value
//! more to be added later

#[cfg(test)]
mod tests;
pub mod firestore_raw;
pub mod firestore;
