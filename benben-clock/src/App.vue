<template>
  <div class="mian_div" >
    <div class="time_box">
      <TaskStatics></TaskStatics>
      <div>
        <transition name="breath-animation">
          <div class="box">
            <span class="centered-text">
              {{ minutes }}:{{ seconds }}
            </span>
          </div>
        </transition>
      </div>
      <div style="position: fixed;padding-top: 35%;padding-left: 25%;">
        <a-button v-if="useTaskStores.runTask?.status === '1'" shape="round" type="primary"
          :style="{ background: 'rgb(0, 100, 200,0.5)' }" @click="stopTask">
          <StopOutlined />
          暂停任务
        </a-button>
        <audio ref="audioPlayer" src="./assets/luomi.flac" style="padding: 50,50;"></audio>
        <a-button v-if="useTaskStores.runTask?.status === '2'" shape="round" type="primary"
          :style="{ background: 'rgb(0, 100, 200,0.5)' }" @click="resTartTask">
          <StarOutlined />
          恢复任务
        </a-button>
      </div>

      <Suspense>
        <AddTask style="position: fixed;padding-top: 35%;padding-left: 1%;"></AddTask>
      </Suspense>

    </div>
    <div class="list_box">
      <Suspense>
        <TaskList></TaskList>
      </Suspense>

    </div>
  </div>
</template>


<script lang="ts" setup>
import { taskStores } from '@/stores/taskStore';
import { computed, onBeforeUnmount, ref, watch } from 'vue';
import AddTask from './cpn/AddTask.vue';
import TaskList from './cpn/TaskList.vue';
import TaskStatics from './cpn/TaskStatics.vue';
import type { TaskInfoDTO } from './dto';
import { StopOutlined, StarOutlined } from '@ant-design/icons-vue';
import { storeToRefs } from 'pinia';
import {start_music,stop_music} from './invoke/ring';



const useTaskStores = taskStores();
const { runTask } = storeToRefs(useTaskStores);
const long_time = 20;
const timeLeft = ref(long_time);
const timer = ref<number | null>(null);







const playSound =  () => {
   start_music().then();
};




// 暂停逻辑要补强
const startTimer = () => {
  timer.value = setInterval(() => {

    if (timeLeft.value > 0) {
      timeLeft.value--;
    } else {
      resetTimer();
      timeLeft.value = long_time;
      useTaskStores.addCashOneTomato();
      stopTask().then(() => {
        playSound();
      });
    }
  }, 1000);
};

const resetTimer = () => {
  clearInterval(timer.value as number);
};

const minutes = computed(() => Math.floor(timeLeft.value / 60).toString().padStart(2, '0'));
const seconds = computed(() => (timeLeft.value % 60).toString().padStart(2, '0'));

const stopTask = async () => {
  if (runTask && runTask.value) {
    await useTaskStores.controlRunTask(runTask?.value as TaskInfoDTO, "2");
    resetTimer();
  }
};

const resTartTask = async () => {
  if (runTask && runTask.value) {
    await useTaskStores.controlRunTask(runTask?.value as TaskInfoDTO, "1");
    startTimer();
  }
};


watch(
  () => useTaskStores.runTask, // 使用函数包裹以确保其响应性
  (newTask, oldTask) => {
    if (newTask && oldTask) {
      if (newTask.no === oldTask.no && newTask.status !== oldTask.status) {
        if (newTask.status === '1') {
          startTimer();
        } else if (newTask.status === '2') {
          resetTimer();
        } else if (newTask.status === '3' || newTask.status === '4') {
          resetTimer();
          timeLeft.value = long_time;
        }
      }
    } else if (newTask) {
      timeLeft.value = long_time;
      startTimer();
    }
  },
  { immediate: true } // { immediate: true } 表示立即触发一次回调函数
);



onBeforeUnmount(() => {
  if (timer.value) {
    clearInterval(timer.value);
  }
});
</script>
<style>
.mian_div {
  width: 100%;
  height: 100%;
}



.list_box {
  position: absolute;
  top: 30%;
  height: 70%;
  width: 100%;
  background-image: url('assets/grass.webp');
  background-repeat: no-repeat;
  /* 不重复图片 */
  background-size: cover;
  /* 覆盖整个元素 */
  background-position: center;
  /* 将图片置于中心 */
}



.box {
  top: 8%;
  left: 2%;
  position: absolute;
  width: 45%;
  height: 50%;
  background-image: url('assets/tomato.webp');
  background-size: cover;
  /* 覆盖整个元素 */
  background-position: center;
  /* 将图片置于中心 */
  background-repeat: no-repeat;
  /* 不重复图片 */
  animation: breathe 3s ease-in-out infinite;
}

.time_box {
  position: absolute;
  left: 0%;
  top: 0%;
  height: 30%;
  width: 100%;
  background-image: url('assets/grass.webp');
  background-size: cover;
  /* 覆盖整个元素 */
  background-position: center;
  /* 将图片置于中心 */
  background-repeat: no-repeat;
  /* 不重复图片 */
}

.centered-text {
  position: absolute;
  color: black;
  top: 65%;
  /* 向下偏移50% */
  left: 50%;
  /* 向右偏移50% */
  transform: translate(-50%, -50%);
  /* 向上和向左偏移自身50%以实现居中 */
}

@keyframes breathe {

  0%,
  100% {
    transform: scale(1);
  }

  50% {
    transform: scale(1.2);
  }
}

.breath-animation-enter-active,
.breath-animation-leave-active {
  animation: breathe 3s ease-in-out infinite;
}
</style>