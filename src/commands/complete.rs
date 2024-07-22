use log::info;

use crate::{config::Config, db::store::Store};

pub fn complete_task(id: &i32) -> Result<(), ()> {
    info!("Completed Task: {}", id);

    let config = Config::new();
    let store = Store::new(&config.dbpath).unwrap();

    store.update_task_status(id, 1)
        

}
