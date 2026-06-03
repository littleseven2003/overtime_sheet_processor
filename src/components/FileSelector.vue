<script setup lang="ts">
import { ref } from "vue";
import { openFileDialog } from "../api/tauri";

const emit = defineEmits<{
  (e: "file-selected", filePath: string, fileName: string): void;
}>();

const fileName = ref("");
const isDragging = ref(false);

async function selectFile() {
  const path = await openFileDialog();
  if (path) {
    fileName.value = path.split("/").pop() || path.split("\\").pop() || "";
    emit("file-selected", path, fileName.value);
  }
}
</script>

<template>
  <div class="file-selector">
    <div
      class="drop-zone"
      :class="{ dragging: isDragging, selected: fileName }"
      @click="selectFile"
      @dragover.prevent="isDragging = true"
      @dragleave="isDragging = false"
      @drop.prevent="isDragging = false"
    >
      <template v-if="!fileName">
        <div class="drop-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" y1="3" x2="12" y2="15"/>
          </svg>
        </div>
        <div class="drop-text">
          <span class="drop-primary">点击选择文件</span>
          <span class="drop-secondary">支持 .xlsx / .xls 格式</span>
        </div>
      </template>
      <template v-else>
        <div class="file-icon">
          <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
          </svg>
        </div>
        <div class="file-info">
          <span class="file-name">{{ fileName }}</span>
          <span class="file-change">点击更换文件</span>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.file-selector {
  width: 100%;
}

.drop-zone {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 16px;
  border: 2px dashed #E0DCD8;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: #FDFCFA;
}

.drop-zone:hover {
  border-color: #D4764E;
  background: #FFF8F5;
}

.drop-zone.dragging {
  border-color: #D4764E;
  background: #FFF0EB;
}

.drop-zone.selected {
  border-style: solid;
  border-color: #D4764E;
  background: #FFF8F5;
}

.drop-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: #F0ECEA;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #B0A8A0;
  flex-shrink: 0;
  transition: all 0.2s ease;
}

.drop-zone:hover .drop-icon {
  background: #FFE8E0;
  color: #D4764E;
}

.drop-text {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.drop-primary {
  font-size: 14px;
  font-weight: 500;
  color: #2D2A26;
}

.drop-secondary {
  font-size: 12px;
  color: #A09890;
}

.file-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: linear-gradient(135deg, #D4764E, #E8956E);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  flex-shrink: 0;
}

.file-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.file-name {
  font-size: 14px;
  font-weight: 500;
  color: #2D2A26;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-change {
  font-size: 12px;
  color: #D4764E;
  cursor: pointer;
}
</style>
