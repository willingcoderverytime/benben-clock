export interface TaskInfoDTO {
    no?: string;
    name: string;
    desc: string;
    hard: string;
    level: string;
    ttype: string;
    status?: string;
    start_time?: string;
    count_tomato?: number;
  }
  

  export interface TaskStatistics {
    finish_task_num: number;
    cost_tomato_num: number;
    un_finish_task_num: number;
    tomorrow_task_num: number;
    // 昨日完成任务
    yesterday_task_num?: number;
  }
  
  export interface GoalsDTO {
    no?: string;
    name: string;
    desc: string;
    start_time: string;
    count_tomato?: number;
  }


  export interface TodoDTO {
    no?: string;
    name: string;
    desc: string;
  }
  
