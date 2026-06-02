<script setup lang="ts">
import { computed } from "vue";
import { NCheckboxGroup, NCheckbox, NButton, NSpace, NText } from "naive-ui";

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

function selectAll() {
  emit("update:modelValue", [...props.labs]);
}

function deselectAll() {
  emit("update:modelValue", []);
}
</script>

<template>
  <div class="lab-selector">
    <n-space vertical>
      <n-text strong>选择研究室</n-text>
      <n-checkbox-group v-model:value="selectedLabs">
        <n-space>
          <n-checkbox
            v-for="lab in labs"
            :key="lab"
            :value="lab"
            :label="lab"
          />
        </n-space>
      </n-checkbox-group>
      <n-space>
        <n-button size="small" @click="selectAll">全选</n-button>
        <n-button size="small" @click="deselectAll">取消全选</n-button>
      </n-space>
    </n-space>
  </div>
</template>

<style scoped>
.lab-selector {
  padding: 12px 0;
}
</style>
