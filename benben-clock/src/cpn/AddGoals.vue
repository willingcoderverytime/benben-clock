<template>
  <div>
    <a-button shape="round" type="primary" :style="{ background: 'rgb(0, 0, 200,0.5)' }" @click="showDrawer">
      <PlusOutlined />
      新增目标
    </a-button>
    <a-modal v-model:open="visible" title="新增目标">
      <a-form :model="form" :rules="rules" layout="vertical">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="目标名称" name="name">
              <a-input v-model:value="form.name" placeholder="输入目标名称" />
            </a-form-item>
          </a-col>
        </a-row>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="开始时间" name="dateTime">
              <a-date-picker v-model:value="form.start_time" :disabled-date="disabledDate" style="width: 100%" />
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
        <a-button type="primary" @click="add_goals">Submit</a-button>
      </div>
    </a-modal>
  </div>
</template>
<script setup lang="ts">
import moment from 'moment';
import { reactive, ref } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue';
import { goalsStores } from '@/stores/goalsStore';
import type { GoalsDTO } from '@/dto';
const useGoalsStores = goalsStores();


// 定义响应式表单状态
const form = reactive({
  name: '',
  start_time: '',
  desc: '',
});

// 定义校验规则
const rules = {
  name: [{ required: true, message: '输入目标名称' }],
  start_time: [{ required: true, message: '选择开始时间', type: 'object' }],
  desc: [{ required: true, message: '目标详细描述' }],
};

// 定义抽屉的可见性
const visible = ref(false);

// 显示抽屉的函数
const showDrawer = () => {
  visible.value = true;
};

const add_goals = () => {
  let xx: string = (form.start_time as any).toDate().toLocaleDateString();
  form.start_time = xx.replace("/", "-").replace("/", "-");
  useGoalsStores.addGoals(form as unknown as GoalsDTO);
  // 清空表单
  for (const key in form) {
    (form as any)[key] = undefined;
  }
  visible.value = false;
}
const disabledDate = (current: any) => {
  // Can not select days before today and today
  return current && current < moment().startOf('day');
};

// 关闭抽屉的函数
const onClose = () => {
  visible.value = false;
};

// add goals


</script>