mod models;

use models::{Vote, Topic};
use std::fs::{self, File};
use std::io::prelude::*;

fn main() {
    let agg_rules = vec!["liquid", "borda", "indirect", "majority"];


    for rule in agg_rules {
        let mut total = 0;
        println!("reading {}", rule);
        let csv_file_name = format!("in/NRM Edges - {}.csv", rule);
        let file = fs::File::open(csv_file_name).unwrap();
        let mut lines = std::io::BufReader::new(file).lines();

        let _ = lines.next(); // discard header

        let mut topics = Vec::new();
        let mut results = Vec::new();

        // init - fill data using the first entry
        let line = lines.next().unwrap().unwrap();
        let (topic_id, _, source, target, num) = unfold_line(&line);
        let mut topic = Topic::new(topic_id);
        let mut vote = Vote::new(source);
        total += num;
        vote.add(target, num);

        // repeat
        for line in lines.flatten() {
            let (topic_id, result_id, source, target, num) = unfold_line(&line);
            if topic.topic_id != topic_id {
                topic.push_vote(vote);
                topics.push(topic);
                topic = Topic::new(topic_id);
                results.push(result_id.to_string());
                vote = Vote::new(source);
            } else if vote.voter_id != source {
                topic.push_vote(vote);
                vote = Vote::new(source);
            }
            vote.add(target, num);
            total += num;
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
        println!("done.");
        println!("total: {}", total);

    }
}

fn unfold_line(line: &str) -> (&str, &str, &str, &str, usize) {
    let mut split =  line.split(',');
    let topic = split.next().unwrap();
    let result = split.next().unwrap();
    let source = split.next().unwrap(); 
    let target = split.next().unwrap();
    let num = split.next().unwrap().parse::<usize>().unwrap();
    
    (topic, result, source, target, num)
}
