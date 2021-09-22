use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt;

#[derive(Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Clone, Debug)]
pub struct SqsMessage {
    body: Option<String>,
}

impl SqsMessage {
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }

    fn as_string(&self) -> String {
        match self.body.clone() {
            Some(s) => s,
            None => String::new(),
        }
    }
}

impl fmt::Display for SqsMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.body.clone() {
            Some(s) => write!(f, "{}", &s),
            None => write!(f, ""),
        }
    }
}

/// Type that accepts a batch of messages from SQS
#[derive(Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Clone, Debug)]
pub struct SqsEvent {
    #[serde(rename = "Records")]
    records: Vec<SqsMessage>,
}

impl SqsEvent {
    /// Convert the batch of SQS Message bodies into your Type
    /// Your type must implement [DeserializeOwned] and [Default]
    ///
    /// # Example
    /// ``` no_run
    /// # type MyType = Vec<String>;
    /// # use lambda_sqs::{SqsEvent, Context};
    /// #
    /// # fn handler(sqs_events:SqsEvent, c: Context) {
    /// let my_events: MyType = sqs_events.into_t();
    /// # }
    /// ```
    pub fn into_t<T>(self) -> Vec<T>
    where
        T: DeserializeOwned + Default,
    {
        self.records
            .into_iter()
            .map(move |message| serde_json::from_str(&message.as_string()).unwrap_or_default())
            .collect()
    }
}

impl fmt::Display for SqsEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[")?;
        self.records
            .clone()
            .into_iter()
            .try_for_each(|item| writeln!(f, "{}", item))?;
        writeln!(f, "]")
    }
}
