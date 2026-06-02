<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import {
  NCard,
  NButton,
  NSpace,
  NText,
  NAlert,
  useMessage,
} from "naive-ui";
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

  // 选择保存位置
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
  <div class="home">
    <n-card title="加班表处理工具" class="main-card">
      <n-space vertical size="large">
        <!-- 文件选择 -->
        <n-card title="选择文件" size="small">
          <FileSelector @file-selected="onFileSelected" />
          <n-alert
            v-if="fileMeta"
            type="info"
            title="文件信息"
            style="margin-top: 8px"
          >
            文件名：{{ fileMeta.file_name }} |
            共 {{ fileMeta.total_rows }} 行数据 |
            发现 {{ fileMeta.labs.length }} 个研究室
          </n-alert>
        </n-card>

        <!-- 研究室选择 -->
        <n-card
          v-if="fileMeta && fileMeta.labs.length > 0"
          title="选择研究室"
          size="small"
        >
          <LabSelector
            :labs="fileMeta.labs"
            v-model="selectedLabs"
          />
        </n-card>

        <!-- 处理按钮 -->
        <n-space justify="center">
          <n-button
            type="primary"
            size="large"
            :loading="processing"
            :disabled="!fileMeta || selectedLabs.length === 0"
            @click="handleProcess"
          >
            {{ processing ? "处理中..." : "开始处理" }}
          </n-button>
        </n-space>

        <!-- 处理日志 -->
        <ProcessLog :logs="logs" />

        <!-- 结果预览 -->
        <ResultTable v-if="resultData.length > 0" :data="resultData" />
      </n-space>
    </n-card>
  </div>
</template>

<style scoped>
.home {
  min-height: 100vh;
  padding: 24px;
  display: flex;
  justify-content: center;
}

.main-card {
  width: 100%;
  max-width: 860px;
}
</style>
