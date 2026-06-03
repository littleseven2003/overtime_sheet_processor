<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { useMessage } from "naive-ui";
import FileSelector from "../components/FileSelector.vue";
import LabSelector from "../components/LabSelector.vue";
import ProcessLog from "../components/ProcessLog.vue";
import ResultTable from "../components/ResultTable.vue";
import {
  readFileMeta,
  saveFileDialog,
  processData,
  onLogEvent,
} from "../api/tauri";
import type { FileMeta, LogEntry } from "../types";

const message = useMessage();

const filePath = ref("");
const fileMeta = ref<FileMeta | null>(null);
const selectedLabs = ref<string[]>([]);
const logs = ref<LogEntry[]>([]);
const processing = ref(false);
const resultData = ref<
  Array<{
    key: number;
    seq: number;
    rewardType: string;
    lab: string;
    name: string;
    overtimeRecords: string;
    reward: string;
    penalty: string;
  }>
>([]);

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await onLogEvent((event) => {
    const now = new Date();
    const time = `${now.getHours().toString().padStart(2, "0")}:${now.getMinutes().toString().padStart(2, "0")}:${now.getSeconds().toString().padStart(2, "0")}`;
    logs.value.push({
      level: event.level as LogEntry["level"],
      message: event.message,
      time,
    });
  });
});

onUnmounted(() => {
  unlisten?.();
});

async function onFileSelected(path: string) {
  filePath.value = path;
  fileMeta.value = null;
  selectedLabs.value = [];
  resultData.value = [];

  try {
    fileMeta.value = await readFileMeta(path);
    message.success(`文件加载成功，发现 ${fileMeta.value.labs.length} 个研究室`);
  } catch (error) {
    message.error(`读取文件失败: ${error}`);
  }
}

async function handleProcess() {
  if (!filePath.value) {
    message.warning("请先选择 Excel 文件");
    return;
  }

  if (selectedLabs.value.length === 0) {
    message.warning("请至少选择一个研究室");
    return;
  }

  const outputPath = await saveFileDialog();
  if (!outputPath) {
    return;
  }

  processing.value = true;
  logs.value = [];
  resultData.value = [];

  try {
    const result = await processData({
      file_path: filePath.value,
      selected_labs: selectedLabs.value,
      output_path: outputPath,
    });

    if (result.success) {
      message.success(`处理完成！共 ${result.total_records} 条记录`);
    }
  } catch (error) {
    message.error(`处理失败: ${error}`);
  } finally {
    processing.value = false;
  }
}
</script>

<template>
  <div class="app-layout">
    <header class="app-header">
      <div class="header-left">
        <div class="app-icon">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
            <line x1="16" y1="13" x2="8" y2="13"/>
            <line x1="16" y1="17" x2="8" y2="17"/>
            <polyline points="10 9 9 9 8 9"/>
          </svg>
        </div>
        <div class="header-text">
          <h1>加班表处理工具</h1>
          <span class="header-subtitle">Overtime Sheet Processor</span>
        </div>
      </div>
      <div class="header-right">
        <span v-if="fileMeta" class="status-badge">
          <span class="status-dot"></span>
          {{ fileMeta.labs.length }} 个研究室 · {{ fileMeta.total_rows }} 行数据
        </span>
      </div>
    </header>

    <main class="app-main">
      <aside class="sidebar">
        <section class="card">
          <div class="card-header">
            <svg class="card-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
            <h3>数据源文件</h3>
          </div>
          <FileSelector @file-selected="onFileSelected" />
        </section>

        <section v-if="fileMeta && fileMeta.labs.length > 0" class="card">
          <div class="card-header">
            <svg class="card-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
              <circle cx="9" cy="7" r="4"/>
              <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
              <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
            </svg>
            <h3>选择研究室</h3>
          </div>
          <LabSelector :labs="fileMeta.labs" v-model="selectedLabs" />
        </section>

        <div class="action-area">
          <button
            class="btn-process"
            :class="{ active: !processing && fileMeta && selectedLabs.length > 0, loading: processing }"
            :disabled="!fileMeta || selectedLabs.length === 0 || processing"
            @click="handleProcess"
          >
            <svg v-if="!processing" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polygon points="5 3 19 12 5 21 5 3"/>
            </svg>
            <svg v-else class="spinner" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 12a9 9 0 1 1-6.219-8.56"/>
            </svg>
            <span>{{ processing ? "处理中..." : "开始处理" }}</span>
          </button>
        </div>
      </aside>

      <section class="content">
        <ProcessLog :logs="logs" />
        <ResultTable v-if="resultData.length > 0" :data="resultData" />
      </section>
    </main>
  </div>
</template>

<style scoped>
.app-layout {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

/* 顶部标题栏 */
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 28px;
  background: var(--color-panel);
  border-bottom: 1px solid var(--color-line);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 14px;
}

.app-icon {
  width: 38px;
  height: 38px;
  background: var(--color-brand);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.header-text h1 {
  font-family: "Fraunces", serif;
  font-size: 19px;
  font-weight: 600;
  color: var(--color-ink);
  line-height: 1.2;
}

.header-subtitle {
  font-size: 12px;
  color: var(--color-faint);
}

.header-right {
  display: flex;
  align-items: center;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 5px 14px;
  background: var(--color-brand-soft);
  border-radius: 20px;
  font-size: 12px;
  color: var(--color-brand);
  font-weight: 500;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: var(--color-success);
  box-shadow: 0 0 6px rgba(8, 145, 178, 0.5);
}

/* 主内容区 */
.app-main {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* 左侧边栏 */
.sidebar {
  width: 340px;
  min-width: 340px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
  background: var(--color-panel-soft);
  border-right: 1px solid var(--color-line);
}

/* 卡片 */
.card {
  background: var(--color-panel);
  border: 1px solid var(--color-line);
  box-shadow: var(--shadow-panel);
  border-radius: 16px;
  padding: 18px;
}

.card-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 14px;
}

.card-icon {
  color: var(--color-brand);
  flex-shrink: 0;
}

.card-header h3 {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-ink);
}

/* 处理按钮 */
.action-area {
  margin-top: auto;
  padding-top: 8px;
}

.btn-process {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 14px 24px;
  border: none;
  border-radius: 14px;
  font-family: "DM Sans", sans-serif;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.25s ease;
  background: var(--color-brand-soft);
  color: var(--color-faint);
  letter-spacing: 0.3px;
}

.btn-process.active {
  background: var(--color-brand);
  color: white;
  box-shadow: 0 6px 20px rgba(37, 99, 235, 0.35);
}

.btn-process.active:hover {
  background: var(--color-brand-strong);
  transform: translateY(-1px);
  box-shadow: 0 8px 28px rgba(37, 99, 235, 0.45);
}

.btn-process.active:active {
  transform: translateY(0);
}

.btn-process:disabled {
  cursor: not-allowed;
}

.btn-process.loading {
  background: var(--color-brand-soft);
  color: var(--color-faint);
  cursor: wait;
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 右侧内容区 */
.content {
  flex: 1;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow: hidden;
}
</style>
