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
      return "#0891b2";
    case "error":
      return "#dc2626";
    default:
      return "#60a5fa";
  }
}

function getLevelBg(level: string) {
  switch (level) {
    case "success":
      return "rgba(8, 145, 178, 0.15)";
    case "error":
      return "rgba(220, 38, 38, 0.15)";
    default:
      return "rgba(96, 165, 250, 0.15)";
  }
}
</script>

<template>
  <div class="log-card">
    <div class="card-header">
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
        <svg width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
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
.log-card {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--color-panel);
  border: 1px solid var(--color-line);
  box-shadow: var(--shadow-panel);
  border-radius: 16px;
  overflow: hidden;
  min-height: 0;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-line);
  flex-shrink: 0;
}

.card-header svg {
  color: var(--color-brand);
}

.card-header h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-ink);
}

.log-count {
  margin-left: auto;
  font-size: 11px;
  font-weight: 600;
  color: var(--color-brand);
  background: var(--color-brand-soft);
  padding: 2px 10px;
  border-radius: 12px;
}

.log-body {
  flex: 1;
  overflow-y: auto;
  padding: 14px 18px;
  font-family: "SF Mono", "Consolas", "Monaco", monospace;
  font-size: 12.5px;
  line-height: 1.7;
  background: #0f172a;
}

.log-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 12px;
  height: 100%;
  color: #334155;
}

.log-empty span {
  font-family: "DM Sans", sans-serif;
  font-size: 13px;
  color: #475569;
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
  padding: 1px 7px;
  border-radius: 4px;
  font-family: "SF Mono", "Consolas", monospace;
  min-width: 34px;
  text-align: center;
  flex-shrink: 0;
}

.log-time {
  color: #475569;
  font-size: 11px;
  min-width: 60px;
  flex-shrink: 0;
}

.log-msg {
  color: #94a3b8;
  word-break: break-all;
}
</style>
