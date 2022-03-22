use reqwest;

pub struct FirestoreRaw {
    pub reqwest_client: reqwest::Client
}

impl FirestoreRaw {
    pub async fn get_document(&self, complete_url: String) -> Result<String, reqwest::Error> {
        let document_get_request_builder = self.reqwest_client.get(complete_url);
        let get_request_result = document_get_request_builder.build();
        let response_result = self.reqwest_client.execute(get_request_result.unwrap()).await;
        let response = response_result.unwrap();
        return Ok(response.text().await.unwrap());
    }
}

#[derive(Default)]
pub struct FirestoreRawBuilder {

}

impl FirestoreRawBuilder {
    pub fn build(&self) -> FirestoreRaw {
        let raw_firestore_return: FirestoreRaw = FirestoreRaw { reqwest_client: reqwest::Client::new() };
        raw_firestore_return
    }
}