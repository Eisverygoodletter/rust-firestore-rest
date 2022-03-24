use reqwest;
use super::structs::{Collection, Document, Firestore};
use async_trait::async_trait;
use serde_json::Value;
#[async_trait]
pub trait RawDataContainer {
    async fn get_raw_string(&self) -> Result<String, reqwest::Error> where Self: Sized {
        let request_builder = self.get_reqwest_client().get(self.get_url());
        let request_to_send = request_builder.build();
        let response_result = self.get_reqwest_client().execute(request_to_send?).await;
        let response = response_result?;
        return Ok(response.text().await?);
    }
    async fn get_complete_json(&self)  -> Result<Value, reqwest::Error> where Self: Sized {
        let raw_string = self.get_raw_string().await;
        let value: Value = serde_json::from_str(&raw_string?).unwrap();
        return Ok(value);
    }
    fn get_reqwest_client(&self) -> &reqwest::Client;
    fn get_url(&self) -> String;
}

impl RawDataContainer for Firestore {
    fn get_reqwest_client(&self) -> &reqwest::Client {
        return &self.reqwest_client;
    }
    fn get_url(&self) -> String {
        return self.db_url.clone();
    }
}

impl RawDataContainer for Collection<'_> {
    fn get_reqwest_client(&self) -> &reqwest::Client {
        return self.parent.get_reqwest_client();
    }
    fn get_url(&self) -> String {
        return format!("{}{}", self.parent.get_url(), self.collection_name);
    }
}

impl RawDataContainer for Document<'_> {
    fn get_reqwest_client(&self) -> &reqwest::Client {
        return self.parent.get_reqwest_client();
    }
    fn get_url(&self) -> String {
        return format!("{}{}", self.parent.get_url(), self.document_name);
    }
}