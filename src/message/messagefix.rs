use std::fs::{self, File};
use std::io::prelude::*;

use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Word2Word {
    Before: String,
    After: String,
}

pub async fn messagefix(data: &String) -> std::io::Result<String> {
    
    let mut ans = String::new();
    ans = data.to_string();

    let in_file = fs::read_to_string("./data/dict.json").expect("JSON READ FAILED");
    let deserialized: Vec<Word2Word> = serde_json::from_str(&in_file).unwrap();

    for dic in &deserialized {
        ans = ans.replace(&dic.Before.to_string(), &dic.After.to_string()); 
    }

    return Ok(ans);
}
