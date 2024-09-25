import type { GoalsDTO } from "@/dto";
import { defineStore } from "pinia";
import { addGoals, query_goals_list, statics_goals } from "../invoke/goals";

interface GoalsState {
  goalsList: GoalsDTO[];
  goalsDetailList: GoalsDTO[];
}

export const goalsStores = defineStore({
  id: "goals-info",
  state: (): GoalsState => ({
    goalsList: [] as GoalsDTO[],
    goalsDetailList: [] as GoalsDTO[],
  }),
  getters: {
    getGoalsList(state): GoalsDTO[] {
      return state.goalsList;
    },
    getGoalsDetailList(state): GoalsDTO[] {
      return state.goalsList;
    },
  },
  actions: {
    addGoals(goals: GoalsDTO) {
     
      addGoals(goals).then(async () => {
        await this.goToRefreshState();
      });
    },
    async goToRefreshState() {
      this.goalsList = await query_goals_list();
      statics_goals().then((res) => {
        this.goalsDetailList = res;
      });
    },
  },
});
