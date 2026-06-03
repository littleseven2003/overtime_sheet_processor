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

// 状态
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

// 日志监听
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

// 文件选择回调
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

// 开始处理
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
    <!-- 顶部标题栏 -->
    <header class="app-header">
      <div class="header-left">
        <div class="app-icon">
          <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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

    <!-- 主内容区：左右分栏 -->
    <main class="app-main">
      <!-- 左侧：操作面板 -->
      <aside class="panel-left">
        <!-- 文件选择 -->
        <section class="section">
          <div class="section-header">
            <svg class="section-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/>
            </svg>
            <h3>数据源文件</h3>
          </div>
          <FileSelector @file-selected="onFileSelected" />
        </section>

        <!-- 研究室选择 -->
        <section v-if="fileMeta && fileMeta.labs.length > 0" class="section">
          <div class="section-header">
            <svg class="section-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/>
              <circle cx="9" cy="7" r="4"/>
              <path d="M23 21v-2a4 4 0 0 0-3-3.87"/>
              <path d="M16 3.13a4 4 0 0 1 0 7.75"/>
            </svg>
            <h3>选择研究室</h3>
          </div>
          <LabSelector :labs="fileMeta.labs" v-model="selectedLabs" />
        </section>

        <!-- 处理按钮 -->
        <div class="action-area">
          <button
            class="process-btn"
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

      <!-- 右侧：日志和结果 -->
      <section class="panel-right">
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
  background-color: #F5F2EE;
}

/* ===== 顶部标题栏 ===== */
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 28px;
  background: linear-gradient(135deg, #2D2A26 0%, #3D3832 100%);
  color: #F5F2EE;
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 14px;
}

.app-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #D4764E, #E8956E);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.header-text h1 {
  font-family: "Fraunces", serif;
  font-size: 20px;
  font-weight: 600;
  letter-spacing: 0.5px;
  line-height: 1.2;
}

.header-subtitle {
  font-size: 12px;
  color: #A09890;
  letter-spacing: 0.3px;
}

.header-right {
  display: flex;
  align-items: center;
}

.status-badge {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 14px;
  background: rgba(255, 255, 255, 0.08);
  border-radius: 20px;
  font-size: 13px;
  color: #C8C0B8;
}

.status-dot {
  width: 7px;
  height: 7px;
  border-radius: 50%;
  background: #4A9D7C;
  box-shadow: 0 0 6px rgba(74, 157, 124, 0.5);
}

/* ===== 主内容区 ===== */
.app-main {
  flex: 1;
  display: flex;
  gap: 0;
  overflow: hidden;
}

/* ===== 左侧面板 ===== */
.panel-left {
  width: 340px;
  min-width: 340px;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow-y: auto;
  border-right: 1px solid #E8E4E0;
  background: #FAFAF8;
}

.section {
  background: white;
  border-radius: 12px;
  padding: 16px;
  border: 1px solid #E8E4E0;
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
}

.section-icon {
  color: #D4764E;
  flex-shrink: 0;
}

.section-header h3 {
  font-size: 13px;
  font-weight: 600;
  color: #2D2A26;
  letter-spacing: 0.2px;
}

/* ===== 处理按钮 ===== */
.action-area {
  margin-top: auto;
  padding-top: 8px;
}

.process-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 14px 24px;
  border: none;
  border-radius: 12px;
  font-family: "DM Sans", sans-serif;
  font-size: 15px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.25s ease;
  background: #D0CCC8;
  color: #8A8580;
  letter-spacing: 0.3px;
}

.process-btn.active {
  background: linear-gradient(135deg, #D4764E, #C06840);
  color: white;
  box-shadow: 0 4px 16px rgba(212, 118, 78, 0.3);
}

.process-btn.active:hover {
  transform: translateY(-1px);
  box-shadow: 0 6px 24px rgba(212, 118, 78, 0.4);
}

.process-btn.active:active {
  transform: translateY(0);
}

.process-btn:disabled {
  cursor: not-allowed;
}

.process-btn.loading {
  background: #D0CCC8;
  color: #8A8580;
  cursor: wait;
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* ===== 右侧面板 ===== */
.panel-right {
  flex: 1;
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
  overflow: hidden;
}
</style>
