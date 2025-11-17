use serde::{Deserialize, Serialize};
use std::{env, fs,io, process};

const todo_list: &str = "Todo_list.json";

#[derive(Debug, Deserialize, Serialize)]
struct task{
description: String,
completed: bool
}

impl Task{
    fn new(description: String) -> task{
        description,
        completed: false,
    }
}

#[derive(Debug, Deserialize, Serialize, Default)];
struct Todolist{
    Task: Vec<task>;
}
impl Todolist{
    fn new() -> Todolist{
        
    }
}
