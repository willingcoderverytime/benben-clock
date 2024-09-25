<template>
    <div>
        <a-button shape="round" type="primary" :style="{ background: 'rgb(0, 0, 200,0.5)' }" @click="showDrawer">
            <PlusOutlined />
            新增任务
        </a-button>
        <a-modal v-model:open="visible" title="新增任务" @ok="add_task">
            <a-form :model="form" :rules="rules" layout="vertical">
                <a-row :gutter="16">
                    <a-col :span="12">
                        <a-form-item label="任务名称" name="name">
                            <a-input v-model:value="form.name" placeholder="输入任务名称" />
                        </a-form-item>
                    </a-col>
                    <a-col :span="12">
                        <a-form-item label="任务分类" name="ttype">
                            <a-select placeholder="选择归属目标" v-model:value="form.ttype">
                                <a-select-option v-for="item in goalsList" value="item.no">{{ item.name}}</a-select-option>
                            </a-select>
                        </a-form-item>
                    </a-col>
                </a-row>
                <a-row :gutter="16">
                    <a-col :span="12">
                        <a-form-item label="难度" name="hard">
                            <a-select placeholder="选择难度" v-model:value="form.hard">
                                <a-select-option value="1">简单</a-select-option>
                                <a-select-option value="2">适中</a-select-option>
                                <a-select-option value="3">困难</a-select-option>
                            </a-select>
                        </a-form-item>
                    </a-col>
                    <a-col :span="12">
                        <a-form-item label="开始时间" name="dateTime">
                            <a-date-picker v-model:value="form.start_time" :disabled-date="disabledDate"
                                style="width: 100%" />
                        </a-form-item>
                    </a-col>
                </a-row>
                <a-row :gutter="16">
                    <a-col :span="12">
                        <a-form-item label="优先级" name="level">
                            <a-select placeholder="选择优先级" v-model:value="form.level">
                                <a-select-option value="1">低</a-select-option>
                                <a-select-option value="2">中</a-select-option>
                                <a-select-option value="3">高</a-select-option>
                            </a-select>
                        </a-form-item>
                    </a-col>
                </a-row>
                <a-row :gutter="16">
                    <a-col :span="24">
                        <a-form-item label="Description" name="desc">
                            <a-textarea v-model:value="form.desc" :rows="4" placeholder="输入描述" />
                        </a-form-item>
                    </a-col>
                </a-row>
            </a-form>
            <div :style="{
                position: 'absolute',
                right: 0,
                bottom: 0,
                width: '100%',
                borderTop: '1px solid #e9e9e9',
                padding: '10px 16px',
                background: '#fff',
                textAlign: 'right',
                zIndex: 1,
            }">
                <a-button style="margin-right: 8px" @click="onClose">Cancel</a-button>
                <a-button type="primary" @click="add_task">Submit</a-button>
            </div>
        </a-modal>
    </div>
</template>
<script setup lang="ts">
import moment from 'moment';
import { reactive, ref } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue';
import { addTask } from '@/invoke/task';
import { taskStores } from '@/stores/taskStore';
import type { TaskInfoDTO } from '@/dto';
import { storeToRefs } from 'pinia';
import { goalsStores } from '@/stores/goalsStore';
const useGoalsStores = goalsStores();

let { goalsList } = storeToRefs(useGoalsStores);

const useTaskStores = taskStores();
// 定义响应式表单状态
const form = reactive({
    name: '',
    ttype: '',
    hard: '',
    level: '',
    start_time: '',
    desc: '',
});

// 定义校验规则
const rules = {
    name: [{ required: true, message: '输入任务名称' }],
    ttype: [{ required: true, message: '选择任务类型' }],
    hard: [{ required: true, message: '预判工作难度' }],
    level: [{ required: true, message: '优先级' }],
    start_time: [{ required: true, message: '选择开始时间', type: 'object' }],
    desc: [{ required: true, message: '任务详细描述' }],
};

// 定义抽屉的可见性
const visible = ref(false);

// 显示抽屉的函数
const showDrawer = () => {
    visible.value = true;
};
const disabledDate = (current: any) => {
    // Can not select days before today and today
    return current && current < moment().startOf('day');
};
// 添加任务的函数
const add_task = async () => {
    let xx: string = (form.start_time as any).toDate().toLocaleDateString();
    form.start_time = xx.replace("/", "-").replace("/", "-");
    addTask(form as unknown as TaskInfoDTO).then(async () => {
        await useTaskStores.refreshTaskList("0");
    })
    // 清空表单
    for (const key in form) {
        (form as any)[key] = undefined;
    }
    visible.value = false;
}

// 关闭抽屉的函数
const onClose = () => {
    visible.value = false;
};
</script>