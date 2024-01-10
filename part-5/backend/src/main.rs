#[macro_use] extern crate rocket;

use rocket::{serde::json::Json, State};
use task_core::Task;
use std::sync::Mutex;
use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

struct TodoState {
    list: Vec<Task>
}

struct ServerState {
    todo: Mutex<TodoState>
}

#[post("/task", format = "json", data = "<task>")]
fn store_task(task: Json<Task>, state: &State<ServerState>) -> &'static str {
    state.todo.lock().unwrap().list.push(task.0);
    "Ok"
}

#[put("/task/<id>", format = "json", data = "<task>")]
fn update_task(id: usize, task: Json<Task>, state: &State<ServerState>) -> &'static str {
    let mut state = state.todo.lock().unwrap();
    let t = state.list.get_mut(id);
    if t.is_none() {
        return "Not found"
    }
    let t = t.unwrap();
    t.message = task.message.clone();
    t.done = task.done;
    "Ok"
}

#[delete("/task/<id>")]
fn delete_task(id: usize, state: &State<ServerState>) -> &'static str {
    state.todo.lock().unwrap().list.remove(id);
    "Ok"
}

#[get("/tasks")]
fn show_tasks(state: &State<ServerState>) -> Json<Vec<Task>> {
    Json(state.todo.lock().unwrap().list.clone())
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Delete, Method::Put]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    rocket::build()
        .attach(cors.to_cors().unwrap())
        .manage(ServerState {
            todo: Mutex::new(TodoState {
                list: vec![]
            })
        })
        .mount("/", routes![store_task, update_task, delete_task, show_tasks])
}


#[cfg(test)]
mod tests {
    use rocket::{local::blocking::Client, http::Status};

    use super::*;

    #[test]
    fn it_works() {
        // Build local test client
        let rocket = rocket();
        let client = Client::tracked(rocket).unwrap();

        // Check if there is 0 tasks in the list
        let req = client.get("/tasks");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_tasks = response.into_json::<Vec<Task>>();
        assert!(response_tasks.is_some());
        let response_tasks = response_tasks.unwrap();
        assert!(response_tasks.len() == 0);

        // Add task to the list
        let task = Task { message: "Todo".to_string(), done: false};
        let req = client.post("/task").json(&task);
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Ok");

        // Check if there's is 1 task in the list
        let req = client.get("/tasks");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_tasks = response.into_json::<Vec<Task>>();
        assert!(response_tasks.is_some());
        let response_tasks = response_tasks.unwrap();
        assert!(response_tasks.len() == 1);
        // Check if the first task has the right message
        let response_task = response_tasks.first().unwrap();
        assert!(response_task.message == task.message);
        assert!(response_task.done == task.done);

        // Modify the task
        let task = Task { message: "Todo Modified".to_string(), done: false};
        let req = client.put(format!("/task/{}", 0)).json(&task);
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Ok");

        // Check if there's is 1 task in the list
        let req = client.get("/tasks");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_tasks = response.into_json::<Vec<Task>>();
        assert!(response_tasks.is_some());
        let response_tasks = response_tasks.unwrap();
        assert!(response_tasks.len() == 1);
        // Check if the first task has the right message
        let response_task = response_tasks.first().unwrap();
        assert!(response_task.message == task.message);
        assert!(response_task.done == task.done);

        // Delete the task
        let req = client.delete(format!("/task/{}", 0)).json(&task);
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Ok");

        // Check if there is 0 tasks in the list
        let req = client.get("/tasks");
        let response = req.dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_tasks = response.into_json::<Vec<Task>>();
        assert!(response_tasks.is_some());
        let response_tasks = response_tasks.unwrap();
        assert!(response_tasks.len() == 0);
    }
}