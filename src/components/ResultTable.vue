<script setup lang="ts">
import { NDataTable } from "naive-ui";
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

defineProps<{
  data: ResultRow[];
}>();

const columns: DataTableColumns<ResultRow> = [
  {
    title: "序号",
    key: "seq",
    width: 50,
    align: "center",
  },
  {
    title: "奖惩项点",
    key: "rewardType",
    width: 120,
    align: "center",
  },
  {
    title: "奖惩明细",
    key: "overtimeRecords",
    width: 180,
    align: "center",
    render(row) {
      const lines = row.overtimeRecords.split("\n");
      return lines.map((line) => `${line}`).join("、");
    },
  },
  {
    title: "建议奖励金额",
    key: "reward",
    width: 100,
    align: "center",
  },
  {
    title: "建议处罚金额",
    key: "penalty",
    width: 100,
    align: "center",
  },
  {
    title: "涉及人员",
    key: "name",
    width: 90,
    align: "center",
  },
  {
    title: "开发室",
    key: "lab",
    width: 130,
    align: "center",
  },
];
</script>

<template>
  <div class="result-panel">
    <div class="result-header">
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <polyline points="9 11 12 14 22 4"/>
        <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/>
      </svg>
      <h3>处理结果</h3>
      <span class="result-count">{{ data.length }} 条记录</span>
    </div>
    <div class="result-body">
      <n-data-table
        :columns="columns"
        :data="data"
        :bordered="false"
        :single-line="false"
        size="small"
        :max-height="220"
      />
    </div>
  </div>
</template>

<style scoped>
.result-panel {
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 12px;
  border: 1px solid #E2E8F0;
  overflow: hidden;
  max-height: 280px;
}

.result-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 18px;
  border-bottom: 1px solid #F1F5F9;
  color: #1E293B;
  flex-shrink: 0;
}

.result-header svg {
  color: #22C55E;
}

.result-header h3 {
  font-size: 14px;
  font-weight: 600;
}

.result-count {
  margin-left: auto;
  font-size: 12px;
  color: #94A3B8;
}

.result-body {
  flex: 1;
  overflow: hidden;
}
</style>
