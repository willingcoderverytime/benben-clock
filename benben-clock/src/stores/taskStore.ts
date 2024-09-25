import type { TaskInfoDTO, TaskStatistics } from "@/dto";
import { defineStore } from "pinia";
import {
  getTodayTaskList,
  control_task,
  addTask,
  addTomoto,
  statics_task,
  getAllTaskList,
} from "../invoke/task";

interface TaskListState {
  taskList: TaskInfoDTO[];
  runTask?: TaskInfoDTO;
  statistics: TaskStatistics;
}
interface TaskListWrap {
  taskList: TaskInfoDTO[];
}

export const taskStores = defineStore({
  id: "task-info",
  state: (): TaskListState => ({
    taskList: [] as TaskInfoDTO[],
    runTask: undefined,
    statistics: { cost_tomato_num: 0 } as TaskStatistics,
  }),
  getters: {
    getTaskList(state): TaskInfoDTO[] {
      console.log(state.taskList);
      return state.taskList;
    },
    getRunTask(state): TaskInfoDTO {
      return state.runTask || ({} as TaskInfoDTO);
    },
    getTaskStatistics(state): TaskStatistics {
      return state.statistics || ({} as TaskStatistics);
    },
  },
  actions: {
    addTaskInfo(task: TaskInfoDTO) {
      addTask(task).then(async () => {
        await this.refreshTaskList("0");
      });
    },
    async refreshTaskList(timeType: string) {
      getAllTaskList({ time: timeType }).then((list) => {
        this.taskList = list;
        let runTask = list.find(
          (task: TaskInfoDTO): boolean => task.status == "1"
        );
        if (timeType === "0") {
          this.runTask = runTask;
        }
      });
      if (timeType === "0") {
        statics_task().then((res) => {
          this.statistics = res;
        });
      }
    },
    async controlRunTask(task: TaskInfoDTO, status: string) {
      if (this.runTask && this.runTask.no === task.no) {
        if (this.runTask.status !== status) {
          control_task(task.no as string, status).then();
        }
      } else if (this.runTask && this.runTask.no !== task.no) {
        control_task(this.runTask.no as string, "2").then(() => {});
        control_task(task.no as string, "1").then(() => {});
      } else {
        control_task(task.no as string, "1").then(() => {});
      }
      task.status = status;
      this.runTask = task;
    },
    addCashOneTomato() {
      if (this.runTask) {
        if (this.runTask.count_tomato) {
          this.runTask.count_tomato += 1;
        } else {
          this.runTask.count_tomato = 1;
        }
        addTomoto(this.runTask.no as string).then(() => {});
      }
      this.statistics.cost_tomato_num += 1;
    },
  },
});
