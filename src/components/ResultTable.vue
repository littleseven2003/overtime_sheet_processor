<script setup lang="ts">
import { NDataTable, NText } from "naive-ui";
import type { DataTableColumns } from "naive-ui";

interface ResultRow {
  key: number;
  seq: number;
  rewardType: string;
  lab: string;
  name: string;
  overtimeRecords: string;
  reward: string;
  penalty: string;
}

const props = defineProps<{
  data: ResultRow[];
}>();

const columns: DataTableColumns<ResultRow> = [
  {
    title: "序号",
    key: "seq",
    width: 60,
    align: "center",
  },
  {
    title: "奖惩项点",
    key: "rewardType",
    width: 140,
    align: "center",
  },
  {
    title: "奖惩明细",
    key: "overtimeRecords",
    width: 200,
    align: "center",
    render(row) {
      const lines = row.overtimeRecords.split("\n");
      return lines.map((line) => `${line}`).join("、");
    },
  },
  {
    title: "建议奖励金额",
    key: "reward",
    width: 120,
    align: "center",
  },
  {
    title: "建议处罚金额",
    key: "penalty",
    width: 120,
    align: "center",
  },
  {
    title: "涉及人员",
    key: "name",
    width: 100,
    align: "center",
  },
  {
    title: "开发室",
    key: "lab",
    width: 160,
    align: "center",
  },
];
</script>

<template>
  <div class="result-table">
    <n-text strong>处理结果预览</n-text>
    <n-data-table
      :columns="columns"
      :data="data"
      :bordered="true"
      :single-line="false"
      size="small"
      :max-height="300"
      style="margin-top: 8px"
    />
  </div>
</template>

<style scoped>
.result-table {
  padding: 12px 0;
}
</style>
