<script setup lang="ts">
import { watch, onMounted, useTemplateRef, markRaw } from "vue";
import * as echarts from "echarts";
import type { CpuInfo } from "@/views/performance/types/index";

const props = defineProps<{
  data: CpuInfo;
  index: number;
}>();

type EChartsOption = echarts.EChartsOption;

const echartsRef = useTemplateRef<HTMLInputElement>("echartsRef");
const xAxisData: number[] = Array.from({ length: 61 }, (v, i) => i - 60);
const yAxisData: number[] = Array.from({ length: 61 }, (v, i) => 0);

watch(
  () => props.data,
  (data) => {
    if (yAxisData.length >= 61) {
      yAxisData.shift();
    }
    yAxisData.push(data.cpu_usage);

    doChart();
  }
);

function doChart() {
  if (!echartsRef.value) return;

  const chartDom = markRaw(echartsRef.value);
  const myChart = echarts.init(chartDom);
  let option: EChartsOption;

  option = {
    xAxis: {
      type: "category",
      boundaryGap: false,
      data: xAxisData,
      axisLabel: {
        formatter: "{value}s",
      },
    },
    yAxis: {
      type: "value",
      max: 100,
      min: 0,
      axisLabel: {
        formatter: "{value}%",
      },
    },
    series: [
      {
        data: yAxisData,
        type: "line",
        animation: false,
        symbol: "none",
      },
    ],
  };

  option && myChart.setOption(option);
}

onMounted(() => {
  doChart();
});
</script>

<template>
  <div ref="echartsRef" class="w-96 h-96 border"></div>
</template>

<style lang="scss"></style>
