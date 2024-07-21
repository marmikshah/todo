use std::iter;

use log::{self, debug, info};
use todo::config::Config;

use crate::db::{
    store::Store,
    task::{Task, TaskParamLengths},
};

fn sfill(str: &String, count: usize) -> String {
    format!("{:<width$}", str, width = count + 1)
}

pub fn list_tasks() {
    debug!("Requesting task list from db");

    let config = Config::new();
    let store = Store::new(&config.dbpath).unwrap();
    let result = store.get_tasks();

    let mut lengths = TaskParamLengths {
        id: 2,
        description: 11,
        status: 11,
    };
    match &result {
        Ok(tasks) => {
            for task in tasks {
                let nchars = &task.description.chars().count();
                if nchars > &lengths.description {
                    lengths.description = *nchars;
                }
            }

            let header = format!(
                "|{}|{}|{}|",
                sfill(&String::from("ID"), lengths.id),
                sfill(&String::from("Description"), lengths.description),
                sfill(&String::from("Status"), lengths.status)
            );

            info!("{}", header);
            let divider = iter::repeat("-")
                .take(header.chars().count())
                .collect::<String>();
            info!("{}", divider);
            for task in tasks {
                info!(
                    "|{}|{}|{}|",
                    sfill(&task.id.to_string(), lengths.id),
                    sfill(&task.description, lengths.description),
                    sfill(&task.status_to_string(), lengths.status)
                )
            }
        }
        Err(_) => {
            panic!("Failed to retrieve tasks");
        }
    }
}
