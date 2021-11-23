mod models;

use models::{Vote, Topic};
use std::fs::{self, File};
use std::io::prelude::*;

fn main() {
    let agg_rules = vec!["liquid", "borda", "indirect", "majority"];

    for rule in agg_rules {
        let csv_file_name = format!("in/NRM Edges - {}.csv", rule);
        let file = fs::File::open(csv_file_name).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();

        println!("{:?}", lines.next()); // discard header

        let mut topics = Vec::new();
        let mut results = Vec::new();

        // init - fill data using the first entry
        let line = lines.next().unwrap().unwrap();
        let split: Vec<String> = line
            .split(',')
            .map(|cs| cs.to_string()).collect();
        let mut topic = Topic::new(&split[0]);
        let mut vote = Vote::new(&split[2]);
        let num = split[4].parse::<usize>().unwrap();
        vote.add(&split[3], num);

        // repeat
        for line in lines.flatten() {
            let split: Vec<String> = line
                .split(',')
                .map(|s| s.to_string()).collect();

            if topic.topic_id != split[0] {
                topics.push(topic);
                topic = Topic::new(&split[0]);
                results.push(split[1].to_string());
                vote = Vote::new(&split[2]);
            } else if vote.voter_id != split[2] {
                topic.push_vote(vote);
                vote = Vote::new(&split[2]);
            }

            let num = split[4].parse::<usize>().unwrap();
            vote.add(&split[3], num);
        }

        //clean
        topic.push_vote(vote);
        topics.push(topic);
        results.push("00000".to_string()); // I don't know the last result, is this important?

        let json_file_name = format!("out/{}.json", rule);
        let buffer = File::create(json_file_name).unwrap();
        let _ = serde_json::to_writer_pretty(buffer, &topics);
        let json_file_results = format!("out/{}-result.json", rule);
        let buffer = File::create(json_file_results).unwrap();
        let _ = serde_json::to_writer_pretty(buffer, &results);
    }
}
