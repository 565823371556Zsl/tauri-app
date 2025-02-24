<script setup lang="ts">
import type { SysInfo, CpuInfo } from "./types/index";
import { listen } from "@tauri-apps/api/event";
import { onUnmounted, ref } from "vue";
import { useThrottleFn } from "@vueuse/core";
import CpuChart from "./components/cpu_chart/index.vue";

const cpus = ref<CpuInfo[]>([]);
const unlisten = await listen<SysInfo>(
  "system_stats",
  useThrottleFn((event) => {
    cpus.value = event.payload.cpus;
  }, 1000)
);
console.log("cpus", cpus);

onUnmounted(() => {
  unlisten();
});
</script>

<template>
  <div>
    <div>preformance页面</div>
    <div class="flex gap-4 flex-wrap">
      <template v-for="(cpuInfo, index) in cpus" :key="index">
        <CpuChart :data="cpuInfo" :index="index"></CpuChart>
      </template>
    </div>
  </div>
</template>

<style lang="scss"></style>
