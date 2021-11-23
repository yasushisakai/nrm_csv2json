use serde::Serialize;
use std::collections::HashMap;

pub type VoteTarget = String;

#[derive(Default, Serialize)]
pub struct Topic {
    #[serde(rename = "topicId")]
    pub topic_id: String,
    pub vote: Vec<Vote>,
}

impl Topic {
    pub fn new(topic_id: &str) -> Self {
        Self {
            topic_id: topic_id.to_string(),
            ..Default::default()
        }
    }

    pub fn push_vote(&mut self, vote: Vote) {
        self.vote.push(vote);
    }
}

#[derive(Default, Serialize)]
pub struct Vote {
    #[serde(rename = "voterId")]
    pub voter_id: String,
    #[serde(rename = "vote")]
    pub voter_points: HashMap<VoteTarget, usize>,
}

impl Vote {
    pub fn new(voter_id: &str) -> Self {
        Self {
            voter_id: voter_id.to_string(),
            ..Default::default()
        }
    }

    pub fn add(&mut self, target: &str, num: usize) {
        self.voter_points.insert(target.to_owned(), num);
    }
}
