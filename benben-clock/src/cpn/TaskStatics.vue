<template>
    <div style="width: 50%; position: fixed; padding-top: 2%; right: 1%; background-color: rgba(185,255,102, 0.6); ">
        <ul style="color:#333;font-family: 'Helvetica Neue', Arial, sans-serif; font-size: 16px;">
            <li>

                执行中 :<a-typography-text :style="visibleTaskName ? { width: '100px' } : { fontSize: '18px' }"
                    :ellipsis="visibleTaskName ? { tooltip: '任务详情!' } : false" :content="runTask?.name"
                    @click="handleVisibleTaskName" />
            </li>
            <li>
                任务详情 :
                <a-typography-text :style="{ width: '100px' }" :ellipsis="{ tooltip: '任务详情!' }" :content="runTask?.desc"
                    @click="handleVisibleTaskDesc" />
                <a-popover v-model:visible="visibleTaskDesc" title="任务详情" trigger="click"
                    :overlay-class-name="'custom-popover'">
                    <template #content>
                        <a-typography-paragraph style="white-space: pre-wrap; right: 1%;">
                            {{ runTask?.desc }}
                        </a-typography-paragraph>
                    </template>
                </a-popover>
            </li>
            <li>
                <span>番茄数量 : {{ runTask?.count_tomato }}</span>
            </li>
        </ul>
        <a-divider style="height: 3px; background-color: #7cb305" />
        <ul style="color: #333; font-family: 'Helvetica Neue', Arial, sans-serif;font-size: 16px; ">

            <li>
                <span>今日完成番茄数量 : {{ statistics?.cost_tomato_num }}</span>
            </li>
            <li>
                <span>任务完成数 : {{ statistics?.finish_task_num }}</span>
            </li>
            <li>
                <span>未完成任务数 : {{ statistics?.un_finish_task_num }}</span>
            </li>
            <li>
                <span>明日任务数 : {{ statistics?.tomorrow_task_num }}</span>
            </li>
        </ul>
    </div>
</template>

<style>
.custom-popover .ant-popover-inner {
    background-color: rgba(183, 240, 113, 0.9);
    right: 5px;
}
</style>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { taskStores } from '@/stores/taskStore';
import { storeToRefs } from 'pinia';
const useTaskStores = taskStores();
const { statistics, runTask } = storeToRefs(useTaskStores);

const visibleTaskName = ref(true);
const visibleTaskDesc = ref(false);
const handleVisibleTaskDesc = () => {
    visibleTaskDesc.value = !visibleTaskDesc.value;
};

const handleVisibleTaskName = () => {
    visibleTaskName.value = !visibleTaskName.value;
};


</script>
