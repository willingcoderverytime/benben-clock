import type { GoalsDTO, TodoDTO } from "@/dto";
import { defineStore } from "pinia";
import { addToTask, addTodo,getAllTodo } from "../invoke/todo";

interface ToDoState {
  todoList: TodoDTO[];
}

export const todoStore = defineStore({
  id: "todo-info",
  state: (): ToDoState => ({
    todoList: [] as TodoDTO[],
  }),
  getters: {
    getTodoList(state): TodoDTO[] {
      return state.todoList;
    },
  },
  actions: {
    addTodo(todo: TodoDTO) {
      addTodo(todo).then(async () => {
        await this.goToRefreshState();
      });
    },
    addToTask(todo: TodoDTO){
      addToTask(todo).then(async () => {
        await this.goToRefreshState();
      });
    },
    async goToRefreshState() {
      this.todoList = await getAllTodo();
    },
  },
});
