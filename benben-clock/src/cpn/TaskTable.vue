<template>

    <a-table :columns="columns" :data-source="taskList" :expandRowByClick="true" :expand-icon-column-index="-1"
        :row-key="(record : any) => record.no" :scroll="{ x: 'calc(350px + 25%)' }"
        class="my-transparent-table custom-expand-icon custom-table-cell">
        <template #action="{ record }">
            <div @click.stop>
                <a key="kaishi" v-if="record.status === '0' || record.status === '2'" @click="startTask(record)">开始</a>
                <a key="wancheng" v-if="record.status !== '4' && (record.status === '0' || record.status === '2')"
                    @click="finishTask(record)">完成</a>
                <a key="quxiao" v-if="record.status !== '3' && (record.status === '0' || record.status === '2')"
                    @click="cancelTask(record)">取消</a>
                <span v-if="record.status === '3'">完成</span>
                <span v-if="record.status === '4'">取消</span>
            </div>
        </template>
        <template #expandedRowRender="{ record }">
            <div style="background-color: rgba(210, 255, 100, 0.2); height: 50px;">
                <a-typography-paragraph  style="white-space: pre-wrap;">
                    {{ record.desc }}
                </a-typography-paragraph>
            </div>
        </template>
        <template #hards="{ text: hard }">
            <span>
                <a-tag :key="hard" :color="hard === '3' ? 'red' : hard === '2' ? 'blue' : 'green'">
                    {{ hard }}
                </a-tag>
            </span>
        </template>
        <template #levels="{ text: level }">
            <span>
                <a-tag :key="level" :color="level === '3' ? 'red' : level === '2' ? 'blue' : 'green'">
                    {{ level }}
                </a-tag>
            </span>
        </template>
    </a-table>
</template>
<style>
.my-transparent-table {
    opacity: 0.65;
}
</style>

<script lang="ts" setup>
import type { TaskInfoDTO } from "@/dto";
import { columns } from "./taskdata";
import { taskStores } from '@/stores/taskStore';
import { storeToRefs } from 'pinia';
import { ref } from "vue";

const timeType = ref("0")
const useTaskStores = taskStores();
await useTaskStores.refreshTaskList(timeType.value);
const { taskList } = storeToRefs(useTaskStores);

const startTask = async (task: TaskInfoDTO) => {
    await useTaskStores.controlRunTask(task, "1");
    useTaskStores.refreshTaskList(timeType.value).then();
};
const cancelTask = async (task: TaskInfoDTO) => {
    await useTaskStores.controlRunTask(task, "4");
    useTaskStores.refreshTaskList(timeType.value).then();

};
const finishTask = async (task: TaskInfoDTO) => {
    await useTaskStores.controlRunTask(task, "3");
    useTaskStores.refreshTaskList(timeType.value).then();
};


</script>
