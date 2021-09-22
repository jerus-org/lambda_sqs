use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SqsMessage {
    body: Option<String>,
}

impl SqsMessage {
    fn as_string(&self) -> String {
        match self.body.clone() {
            Some(s) => s,
            None => String::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SqsEvent {
    #[serde(rename = "Records")]
    records: Vec<SqsMessage>,
}

impl SqsEvent {
    pub fn to_vec<T>(self) -> Vec<T>
    where
        T: DeserializeOwned + Default,
    {
        self
            .records
            .into_iter()
            .map(move |message| serde_json::from_str(&message.as_string()).unwrap_or_default())
            .collect()
    }
}
