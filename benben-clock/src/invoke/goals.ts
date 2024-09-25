import type { GoalsDTO } from "@/dto";
import { invoke } from "@tauri-apps/api/tauri";

export function addGoals(goals: GoalsDTO) {
  return invoke<String>("add_goals", { goals: goals });
}

export function query_goals_list() {
  return invoke<GoalsDTO[]>("query_goals", { });
}




export function statics_goals() {
  return invoke<GoalsDTO[]>("query_goals_detail", {});
}

