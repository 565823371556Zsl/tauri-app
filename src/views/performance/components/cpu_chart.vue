<script setup lang="ts">
import {
  watch,
  onMounted,
  useTemplateRef,
  markRaw,
  shallowRef,
  onUnmounted,
} from "vue";
import * as echarts from "echarts";
import type { CpuInfo } from "@/views/performance/types/index";

const props = defineProps<{
  data: CpuInfo;
  index: number;
}>();

type EChartsOption = echarts.EChartsOption;

const echartsRef = useTemplateRef<HTMLInputElement>("echartsRef");
const myChart = shallowRef<echarts.ECharts>();
const xAxisData: number[] = Array.from({ length: 61 }, (_, i) => i - 60);
const yAxisData: number[] = Array.from({ length: 61 }, () => 0);

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
  if (!myChart.value) {
    myChart.value = echarts.init(chartDom);
  }
  let option: EChartsOption;

  option = {
    xAxis: {
      type: "category",
      boundaryGap: false,
      data: xAxisData,
      axisLabel: {
        formatter: "{value}",
      },
      splitLine: {
        show: true,
        showMaxLine: true,
        showMinLine: true,
      },
    },
    yAxis: {
      type: "value",
      max: 100,
      min: 0,
      axisLabel: {
        formatter: "{value}",
      },
      nameLocation: "end",
      nameTextStyle: {
        color: "#666",
        padding: [0, 0, -5, 45], // 调整位置
      },
      name: "%利用率",
    },
    series: [
      {
        data: yAxisData,
        type: "line",
        animation: false,
        symbol: "none",
        areaStyle: {
          color: "rgba(135, 154, 215, 0.8)",
          origin: "start", // 填充从坐标轴开始
        },
      },
    ],
    title: {
      left: "center",
      text: `CPU${props.index + 1}(${props.data.frequency}MHZ)`,
    },
    grid: {
      left: 0, // 左侧边距设为0
      right: "1%", // 右侧边距设为0
      top: "15%",
      bottom: "0%",
      containLabel: true, // 确保坐标轴标签在grid区域内
    },
  };

  option && myChart.value.setOption(option);
}

onMounted(() => {
  doChart();
});

onUnmounted(() => {
  if (myChart.value) {
    myChart.value.dispose();
  }
});
</script>

<template>
  <div class="relative p-4 border" style="width: 35rem; height: 20rem">
    <div class="h-full w-full" ref="echartsRef"></div>
  </div>
</template>

<style lang="scss"></style>
