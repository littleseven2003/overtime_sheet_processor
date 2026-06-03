<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import type { LogEntry } from "../types";

const props = defineProps<{
  logs: LogEntry[];
}>();

const logContainer = ref<HTMLElement>();

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
      return "#22C55E";
    case "error":
      return "#EF4444";
    default:
      return "#60A5FA";
  }
}

function getLevelBg(level: string) {
  switch (level) {
    case "success":
      return "rgba(34, 197, 94, 0.12)";
    case "error":
      return "rgba(239, 68, 68, 0.12)";
    default:
      return "rgba(96, 165, 250, 0.12)";
  }
}
</script>

<template>
  <div class="log-panel">
    <div class="log-header">
      <svg width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
        <polyline points="14 2 14 8 20 8"/>
        <line x1="16" y1="13" x2="8" y2="13"/>
        <line x1="16" y1="17" x2="8" y2="17"/>
      </svg>
      <h3>处理日志</h3>
      <span v-if="logs.length > 0" class="log-count">{{ logs.length }}</span>
    </div>
    <div ref="logContainer" class="log-body">
      <div v-if="logs.length === 0" class="log-empty">
        <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
          <polyline points="14 2 14 8 20 8"/>
        </svg>
        <span>等待处理...</span>
      </div>
      <div
        v-for="(log, index) in logs"
        :key="index"
        class="log-line"
      >
        <span
          class="log-badge"
          :style="{ color: getLevelColor(log.level), background: getLevelBg(log.level) }"
        >
          {{ log.level === "info" ? "INFO" : log.level === "success" ? "OK" : "ERR" }}
        </span>
        <span class="log-time">{{ log.time }}</span>
        <span class="log-msg">{{ log.message }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.log-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 12px;
  border: 1px solid #E2E8F0;
  overflow: hidden;
  min-height: 0;
}

.log-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 18px;
  border-bottom: 1px solid #F1F5F9;
  color: #1E293B;
  flex-shrink: 0;
}

.log-header svg {
  color: #3B82F6;
}

.log-header h3 {
  font-size: 14px;
  font-weight: 600;
}

.log-count {
  margin-left: auto;
  font-size: 11px;
  font-weight: 600;
  color: #3B82F6;
  background: rgba(59, 130, 246, 0.1);
  padding: 2px 8px;
  border-radius: 10px;
}

.log-body {
  flex: 1;
  overflow-y: auto;
  padding: 12px 16px;
  font-family: "SF Mono", "Consolas", "Monaco", monospace;
  font-size: 12.5px;
  line-height: 1.7;
  background: #1E293B;
}

.log-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  height: 100%;
  color: #475569;
}

.log-empty span {
  font-family: "DM Sans", sans-serif;
  font-size: 13px;
  color: #64748B;
}

.log-line {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 2px 0;
  animation: fadeIn 0.15s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(4px); }
  to { opacity: 1; transform: translateY(0); }
}

.log-badge {
  font-size: 10px;
  font-weight: 700;
  padding: 1px 6px;
  border-radius: 4px;
  font-family: "SF Mono", "Consolas", monospace;
  min-width: 32px;
  text-align: center;
  flex-shrink: 0;
}

.log-time {
  color: #64748B;
  font-size: 11px;
  min-width: 60px;
  flex-shrink: 0;
}

.log-msg {
  color: #CBD5E1;
  word-break: break-all;
}
</style>
