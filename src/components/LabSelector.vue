<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  labs: string[];
  modelValue: string[];
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string[]): void;
}>();

const selectedLabs = computed({
  get: () => props.modelValue,
  set: (val) => emit("update:modelValue", val),
});

function toggle(lab: string) {
  const current = [...selectedLabs.value];
  const idx = current.indexOf(lab);
  if (idx >= 0) {
    current.splice(idx, 1);
  } else {
    current.push(lab);
  }
  emit("update:modelValue", current);
}

function selectAll() {
  emit("update:modelValue", [...props.labs]);
}

function deselectAll() {
  emit("update:modelValue", []);
}

function isSelected(lab: string) {
  return selectedLabs.value.includes(lab);
}
</script>

<template>
  <div class="lab-selector">
    <div class="lab-grid">
      <button
        v-for="lab in labs"
        :key="lab"
        class="lab-chip"
        :class="{ selected: isSelected(lab) }"
        @click="toggle(lab)"
      >
        <svg v-if="isSelected(lab)" class="check-icon" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
          <polyline points="20 6 9 17 4 12"/>
        </svg>
        <span>{{ lab }}</span>
      </button>
    </div>
    <div class="lab-actions">
      <button class="link-btn" @click="selectAll">全选</button>
      <span class="link-divider">·</span>
      <button class="link-btn" @click="deselectAll">取消全选</button>
      <span v-if="selectedLabs.length > 0" class="selected-count">
        已选 {{ selectedLabs.length }} 个
      </span>
    </div>
  </div>
</template>

<style scoped>
.lab-selector {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.lab-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.lab-chip {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  border: 1.5px solid #E2E8F0;
  border-radius: 8px;
  background: #F8FAFC;
  font-family: "DM Sans", sans-serif;
  font-size: 13px;
  font-weight: 500;
  color: #475569;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.lab-chip:hover {
  border-color: #3B82F6;
  color: #3B82F6;
  background: #EFF6FF;
}

.lab-chip.selected {
  border-color: #3B82F6;
  background: linear-gradient(135deg, #3B82F6, #2563EB);
  color: white;
}

.check-icon {
  flex-shrink: 0;
}

.lab-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.link-btn {
  background: none;
  border: none;
  font-family: "DM Sans", sans-serif;
  font-size: 12px;
  color: #94A3B8;
  cursor: pointer;
  padding: 2px 0;
  transition: color 0.2s;
}

.link-btn:hover {
  color: #3B82F6;
}

.link-divider {
  color: #CBD5E1;
  font-size: 12px;
}

.selected-count {
  margin-left: auto;
  font-size: 12px;
  color: #3B82F6;
  font-weight: 500;
}
</style>
