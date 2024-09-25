<template>


    <a-tabs v-model:activeKey="timeType" @change="onTabClick">
        <a-tab-pane key="0" tab="今日任务">
            <!-- 表格1 -->
            <TaskTable></TaskTable>
        </a-tab-pane>
        <a-tab-pane key="1" tab="本周任务">
            <!-- 表格2 -->
            <TaskTable></TaskTable>
        </a-tab-pane>
        <a-tab-pane key="2" tab="本月任务">
            <!-- 表格2 -->
            <TaskTable></TaskTable>
        </a-tab-pane>
        <a-tab-pane key="3" tab="上周完成">
            <!-- 表格2 -->
            <TaskTable></TaskTable>
        </a-tab-pane>
        <a-tab-pane key="4" tab="远期目标">
            <!-- 表格2 -->
            <GoalsTable></GoalsTable>
        </a-tab-pane>
        <a-tab-pane key="5" tab="TODO">
            <!-- 表格2 -->
            <TodoTable></TodoTable>
        </a-tab-pane>
    </a-tabs>

</template>
<style>
.my-transparent-table {
    opacity: 0.75;
}
</style>

<script lang="ts" setup>
import TaskTable from "./TaskTable.vue";
import TodoTable from "./TodoTable.vue";
import GoalsTable from "./GoalsTable.vue";
import { ref } from "vue";
import { taskStores } from '@/stores/taskStore';
import { goalsStores } from '@/stores/goalsStore';
import { todoStore } from '@/stores/todoStore';

const timeType = ref("0")
const useTaskStores = taskStores();
await useTaskStores.refreshTaskList(timeType.value);


const useGoalsStores = goalsStores();
await useGoalsStores.goToRefreshState();

const useTodoStores = todoStore();
await useTodoStores.goToRefreshState();

const onTabClick = async (key: string) => {
    if (key !== "4"&& key !== "5" ) {
        timeType.value = key;
        await useTaskStores.refreshTaskList(timeType.value);
    }
}


</script>
