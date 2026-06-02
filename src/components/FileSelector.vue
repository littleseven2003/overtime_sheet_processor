<script setup lang="ts">
import { ref } from "vue";
import { NButton, NText, NSpace } from "naive-ui";
import { openFileDialog } from "../api/tauri";

const emit = defineEmits<{
  (e: "file-selected", filePath: string, fileName: string): void;
}>();

const fileName = ref("");
const filePath = ref("");

async function selectFile() {
  const path = await openFileDialog();
  if (path) {
    filePath.value = path;
    // 从路径中提取文件名
    fileName.value = path.split("/").pop() || path.split("\\").pop() || "";
    emit("file-selected", path, fileName.value);
  }
}
</script>

<template>
  <div class="file-selector">
    <n-space align="center">
      <n-button type="primary" @click="selectFile">选择 Excel 文件</n-button>
      <n-text v-if="fileName" depth="3">
        {{ fileName }}
      </n-text>
      <n-text v-else depth="3" italic>
        未选择文件
      </n-text>
    </n-space>
  </div>
</template>

<style scoped>
.file-selector {
  padding: 12px 0;
}
</style>
