use serde::Serialize;

use crate::model::Todo;

// use crate::model::Todo;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct TodoListResponse {
    pub status: String,
    pub data: Vec<Todo>,
}

#[derive(Serialize, Debug)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: Todo,
}

// #[derive(Serialize, Debug)]
// pub struct TodoData {
//     pub todo: Todo,
// }
//
// #[derive(Serialize, Debug)]
// pub struct SingleTodoResponse {
//     pub status: String,
//     pub data: TodoData,
// }
//
// #[derive(Serialize, Debug)]
// pub struct TodoListResponse {
//     pub status: String,
//     pub results: usize,
//     pub todos: Vec<Todo>,
// }
