<template>
  <div>
    <a-button shape="round" type="primary" :style="{ background: 'rgb(0, 0, 200,0.5)' }" @click="showDrawer">
      <PlusOutlined />
      新增目标
    </a-button>
    <a-modal v-model:open="visible" title="新增TODO">
      <a-form :model="form" :rules="rules" layout="vertical">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item label="TODO名称" name="name">
              <a-input v-model:value="form.name" placeholder="输入TODO名称" />
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
        <a-button type="primary" @click="add_todo">Submit</a-button>
      </div>
    </a-modal>
  </div>
</template>
<script setup lang="ts">
import { reactive, ref } from 'vue';
import { PlusOutlined } from '@ant-design/icons-vue';
import { todoStore } from '@/stores/todoStore';
import type { TodoDTO } from '@/dto';
const useTodoStore = todoStore();


// 定义响应式表单状态
const form = reactive({
  name: '',
  desc: '',
});

// 定义校验规则
const rules = {
  name: [{ required: true, message: '输入目标名称' }],
  desc: [{ required: true, message: '目标详细描述' }],
};

// 定义抽屉的可见性
const visible = ref(false);

// 显示抽屉的函数
const showDrawer = () => {
  visible.value = true;
};

const add_todo = () => {
  useTodoStore.addTodo(form as unknown as TodoDTO);
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

// add goals


</script>