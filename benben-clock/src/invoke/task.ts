import type { TaskInfoDTO, TaskStatistics } from "@/dto";
import { invoke } from "@tauri-apps/api/tauri";

export function addTask(task: TaskInfoDTO) {
  return invoke<String>("add_task", { task: task });
}

export interface TaskInfoQuery {
  status?: string;
  time: string; // 0 今天  1 本周 2 本月  3 本年
}

export function getAllTaskList(task_query: TaskInfoQuery) {
  return invoke<TaskInfoDTO[]>("query_task", {
    query: task_query,
  });
}

export function getTodayTaskList() {
  let query = { time: "0" };
  return invoke<TaskInfoDTO[]>("query_task", {
    query: query,
  });
}

export function control_task(no: string, action: string) {
  let control = { no: no, action: action };
  return invoke<TaskInfoDTO[]>("control_task", {
    action: control,
  });
}

export function statics_task() {
  return invoke<TaskStatistics>("query_task_statics", {});
}

export function addTomoto(no: string) {
  return invoke<TaskStatistics>("add_tomato_task", { no: no });
}
