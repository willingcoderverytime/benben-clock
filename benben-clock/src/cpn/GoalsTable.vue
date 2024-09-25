<template>
    <a-table :columns="columns" :data-source="goalsDetailList" :expand-icon-column-index="-1"
        :row-key="(record: any) => record.no" :scroll="{ x: 'calc(350px + 25%)' }"
        class="my-transparent-table custom-expand-icon custom-table-cell">
        <template #expandedRowRender="{ record }">
            <div style="background-color: rgba(210, 255, 100, 0.2); height: 50px;">
                <a-typography-paragraph style="white-space: pre-wrap;">
                    {{ record.desc }}
                </a-typography-paragraph>
            </div>
        </template>
    </a-table>
    <AddGoals></AddGoals>
</template>
<style>
.my-transparent-table {
    opacity: 0.65;
}
</style>

<script lang="ts" setup>
import { goalsStores } from '@/stores/goalsStore';
import { storeToRefs } from 'pinia';
import AddGoals from './AddGoals.vue';
const goalsStore = goalsStores();
const { goalsDetailList } = storeToRefs(goalsStore);

const columns = [
    {
        title: "目标名称",
        dataIndex: "name",
        align: "center",
        key: "name",
        width: 70,
        fixed: "left",
        customHeaderCell: () => {
            return {
                style: {
                    padding: "5px",
                },
            };
        },
        customCell: () => {
            return {
                style: {
                    padding: "5px",
                },
            };
        },
    },
    {
        title: "开始时间",
        dataIndex: "start_time",
        key: "start_time",
        width: 50,
        align: "center",
        customHeaderCell: () => {
            return {
                style: {
                    padding: "5px",
                },
            };
        },
        customCell: () => {
            return { style: { padding: "5px" } };
        },
    },
    {
        title: "花费时间",
        key: "count_tomato",
        align: "center",
        ellipsis: true,
        dataIndex: "decount_tomato",
        width: 28,
        customHeaderCell: () => {
            return {
                style: {
                    padding: "5px",
                },
            };
        },
        customCell: () => {
            return { style: { padding: "5px" } };
        },
    },
];


</script>
