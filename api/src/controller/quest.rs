use rocket::State;
use rocket_contrib::json::Json;
use std::{collections::HashMap, sync::Mutex};

use crate::{model::quest::Quest, service::quest};

#[rocket::post("/quest", format = "application/json", data = "<data>")]
pub fn create(service: State<Mutex<quest::Service>>, data: Json<Quest>) -> Result<Json<Quest>, String> {
    let quest = service.lock().unwrap().create(data.into_inner().into_text());
    match quest {
        Ok(q) => return Ok(Json(q)),
        Err(error) => return Err(error),
    };
}

#[rocket::put("/quest", format = "application/json", data = "<data>")]
pub fn update(service: State<Mutex<quest::Service>>, data: Json<Quest>) -> Result<Json<Quest>, String> {
    let quest = service.lock().unwrap().update(data.into_inner());
    match quest {
        Ok(q) => return Ok(Json(q)),
        Err(error) => return Err(error),
    };
}

// NOTE: looking for a solution to ignore the warning
#[rocket::get("/quest", format = "application/json", data = "<data>")]
pub fn get(service: State<Mutex<quest::Service>>, data: Json<Quest>) -> Result<Json<Quest>, String> {
    let quest = service.lock().unwrap().get(data.into_inner());
    match quest {
        Ok(q) => return Ok(Json(q)),
        Err(error) => return Err(error),
    };
}

#[rocket::get("/quests")]
pub fn list(service: State<Mutex<quest::Service>>) -> Result<Json<HashMap<uuid::Uuid, Quest>>, String> {
    let quests = service.lock().unwrap().list();
    match quests {
        Ok(q) => return Ok(Json(q)),
        Err(error) => return Err(error),
    };
}

#[rocket::delete("/quest", format = "application/json", data = "<data>")]
pub fn delete(service: State<Mutex<quest::Service>>, data: Json<Quest>) -> Result<(), String> {
    service.lock().unwrap().delete(data.into_inner())?;
    Ok(())
}
