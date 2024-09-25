<template>
    <a-table :columns="columns" :data-source="todoList" 
        :row-key="(record: any) => record.no" :scroll="{ x: 'calc(350px + 25%)' }"
        class="my-transparent-table custom-expand-icon custom-table-cell">
        <template #action="{ record }">
            <div @click.stop>
                <a key="wancheng" @click="addToTask(record)">转任务</a>
            </div>
        </template>
    </a-table>
    <AddTodo></AddTodo>
</template>
<style>
.my-transparent-table {
    opacity: 0.65;
}
</style>

<script lang="ts" setup>
import { todoStore } from '@/stores/todoStore';
import { storeToRefs } from 'pinia';
import AddTodo from './AddTodo.vue';
const useTodoStore = todoStore();

const { todoList } = storeToRefs(useTodoStore);
function addToTask(param:any){
    useTodoStore.addToTask(param);
}
const columns = [
    {
        title: "名称",
        dataIndex: "name",
        align: "center",
        key: "name",
        width: 100,
    },
    {
        title: "描述",
        dataIndex: "desc",
        key: "desc",
        width: 200,
        align: "center",
    },
    {
    title: "操作",
    ellipsis: true,
    dataIndex: "",
    key: "x",
    slots: { customRender: "action" },
    width: 60,
    align: "center",
    fixed: "right",
    customHeaderCell: () => {
      return {
        style: {
          padding: "5px",
        },
      };
    },
    customCell: () => {
      return { style: { padding: "8px" } };
    },
  },
];


</script>
