import type { TodoDTO } from "@/dto";
import { invoke } from "@tauri-apps/api/tauri";


export function addTodo(todo: TodoDTO) {
  return invoke<String>("add_todo", {
    info: todo,
  });
}

export function addToTask(todo: TodoDTO) {
  return invoke<String>("add_to_task", {
    info: todo,
  });
}

export function getAllTodo() {
  let query = { time: "0" };
  return invoke<TodoDTO[]>("query_todo", {
  });
}

