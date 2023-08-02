mod util;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Uri(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub id: Uri,
    pub entity: Uri,
    pub field: Uri,
    pub source: Uri,
    pub value: String,
    #[serde(with = "util::serde::iso8601")]
    pub stated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFact {
    pub entity: Uri,
    pub field: Uri,
    pub source: Uri,
    pub value: String,
    #[serde(with = "util::serde::iso8601")]
    pub stated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateFactsReq {
    pub facts: Vec<UserFact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateFactsRes {
    pub facts: Vec<Fact>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub uri: Uri,
    pub fields: HashMap<Uri, String>,
    pub prior_facts: HashMap<Uri, Fact>,
    #[serde(with = "util::serde::iso8601")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "util::serde::iso8601")]
    pub last_updated_at: DateTime<Utc>,
}

impl Entity {
    pub fn new(uri: Uri) -> Self {
        Self {
            uri,
            created_at: chrono::Utc::now(),
            last_updated_at: chrono::Utc::now(),
            fields: Default::default(),
            prior_facts: Default::default(),
        }
    }

    pub fn consolidate(&mut self, fact: Fact) {
        if let Some(prior_fact) = self.prior_facts.get(&fact.field) {
            if fact.stated_at >= prior_fact.stated_at {
                self.insert_fact(fact);
            }
        } else {
            self.insert_fact(fact);
        }
    }

    fn insert_fact(&mut self, fact: Fact) {
        self.fields.insert(fact.field.clone(), fact.value.clone());
        self.prior_facts.insert(fact.field.clone(), fact);
        self.last_updated_at = chrono::Utc::now();
    }
}
