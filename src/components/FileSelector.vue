<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { openFileDialog } from "../api/tauri";
import { getCurrentWindow } from "@tauri-apps/api/window";

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

function handleDrop(paths: string[]) {
  isDragging.value = false;
  if (paths.length > 0) {
    const path = paths[0];
    const ext = path.split(".").pop()?.toLowerCase();
    if (ext === "xlsx" || ext === "xls") {
      fileName.value = path.split("/").pop() || path.split("\\").pop() || "";
      emit("file-selected", path, fileName.value);
    }
  }
}

let unlisten: (() => void) | null = null;

onMounted(async () => {
  const appWindow = getCurrentWindow();
  unlisten = await appWindow.onDragDropEvent((event) => {
    if (event.payload.type === "drop") {
      handleDrop(event.payload.paths);
    } else if (event.payload.type === "over") {
      isDragging.value = true;
    } else if (event.payload.type === "leave") {
      isDragging.value = false;
    }
  });
});

onUnmounted(() => {
  unlisten?.();
});
</script>

<template>
  <div class="file-selector">
    <div
      class="drop-zone"
      :class="{ dragging: isDragging, selected: fileName }"
      @click="selectFile"
    >
      <template v-if="!fileName">
        <div class="drop-icon">
          <svg width="30" height="30" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
            <polyline points="17 8 12 3 7 8"/>
            <line x1="12" y1="3" x2="12" y2="15"/>
          </svg>
        </div>
        <div class="drop-text">
          <span class="drop-primary">点击选择文件</span>
          <span class="drop-secondary">支持 .xlsx / .xls 格式，也可拖拽文件到此处</span>
        </div>
      </template>
      <template v-else>
        <div class="file-icon">
          <svg width="26" height="26" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
            <polyline points="14 2 14 8 20 8"/>
          </svg>
        </div>
        <div class="file-info">
          <span class="file-name">{{ fileName }}</span>
          <span class="file-change">点击更换文件，或拖拽新文件到此处</span>
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
  border: 2px dashed var(--color-line);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s ease;
  background: var(--color-panel-soft);
}

.drop-zone:hover {
  border-color: var(--color-brand);
  background: var(--color-brand-soft);
}

.drop-zone.dragging {
  border-color: var(--color-brand);
  background: var(--color-brand-soft);
  border-style: solid;
  transform: scale(1.01);
}

.drop-zone.selected {
  border-style: solid;
  border-color: var(--color-brand);
  background: var(--color-brand-soft);
}

.drop-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: var(--color-brand-soft);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-brand);
  flex-shrink: 0;
  transition: all 0.2s ease;
}

.drop-zone:hover .drop-icon {
  background: var(--color-brand);
  color: white;
}

.drop-text {
  display: flex;
  flex-direction: column;
  gap: 3px;
}

.drop-primary {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-ink);
}

.drop-secondary {
  font-size: 12px;
  color: var(--color-faint);
}

.file-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  background: var(--color-brand);
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
  color: var(--color-ink);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.file-change {
  font-size: 12px;
  color: var(--color-brand);
  cursor: pointer;
}
</style>
