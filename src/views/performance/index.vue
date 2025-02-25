<script setup lang="ts">
import type { SysInfo, CpuInfo, MemoryInfo } from "./types/index";
import { listen } from "@tauri-apps/api/event";
import type { EventCallback } from "@tauri-apps/api/event";
import { onUnmounted, ref } from "vue";
import { useThrottleFn } from "@vueuse/core";
import CpuChart from "./components/cpu_chart.vue";

const cpus = ref<CpuInfo[]>([]);
const memory = ref<MemoryInfo>();
const unlisten = await listen(
  "system_stats",
  useThrottleFn<EventCallback<SysInfo>>((event) => {
    console.log("信息更新");
    cpus.value = event.payload.cpus;
    memory.value = event.payload.memory;
  }, 1000)
);

console.log("cpus", cpus);

onUnmounted(() => {
  unlisten();
});
</script>

<template>
  <div class="w-full h-screen flex">
    <el-menu
      default-active="2"
      class="el-menu-vertical-demo"
      @open="handleOpen"
      @close="handleClose"
    >
      <el-menu-item index="1">
        <span>CPU</span>
      </el-menu-item>
      <el-menu-item index="2">
        <span>内存</span>
      </el-menu-item>
      <el-menu-item index="3">
        <span>硬盘</span>
      </el-menu-item>
      <el-menu-item index="4">
        <span>进程</span>
      </el-menu-item>
    </el-menu>
    <div class="h-full overflow-y-auto">
      <div class="flex gap-4 flex-wrap">
        <template v-for="(cpuInfo, index) in cpus" :key="index">
          <CpuChart :data="cpuInfo" :index="index"></CpuChart>
        </template>
      </div>
    </div>
  </div>
</template>

<style lang="scss"></style>
