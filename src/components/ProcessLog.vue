<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import { NCard, NText } from "naive-ui";
import type { LogEntry } from "../types";

const props = defineProps<{
  logs: LogEntry[];
}>();

const logContainer = ref<HTMLElement>();

// 自动滚动到底部
watch(
  () => props.logs.length,
  async () => {
    await nextTick();
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight;
    }
  }
);

function getLevelColor(level: string) {
  switch (level) {
    case "success":
      return "#18a058";
    case "error":
      return "#d03050";
    default:
      return "#2080f0";
  }
}
</script>

<template>
  <n-card title="处理日志" size="small">
    <div ref="logContainer" class="log-container">
      <div
        v-for="(log, index) in logs"
        :key="index"
        class="log-line"
      >
        <n-text :style="{ color: getLevelColor(log.level) }">
          [{{ log.level.toUpperCase() }}]
        </n-text>
        <n-text depth="3" class="log-time">{{ log.time }}</n-text>
        <n-text>{{ log.message }}</n-text>
      </div>
      <div v-if="logs.length === 0" class="empty-log">
        <n-text depth="3" italic>暂无日志</n-text>
      </div>
    </div>
  </n-card>
</template>

<style scoped>
.log-container {
  height: 200px;
  overflow-y: auto;
  background-color: #1e1e1e;
  border-radius: 4px;
  padding: 12px;
  font-family: "Consolas", "Monaco", monospace;
  font-size: 13px;
  line-height: 1.6;
}

.log-line {
  display: flex;
  gap: 8px;
  align-items: baseline;
}

.log-time {
  font-size: 12px;
  min-width: 80px;
}

.empty-log {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}
</style>
