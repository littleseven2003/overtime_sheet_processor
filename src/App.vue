<script setup lang="ts">
import Home from "./views/Home.vue";
</script>

<template>
  <n-config-provider :theme-overrides="themeOverrides">
    <n-message-provider>
      <Home />
    </n-message-provider>
  </n-config-provider>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { NConfigProvider, NMessageProvider } from "naive-ui";

export default defineComponent({
  components: {
    NConfigProvider,
    NMessageProvider,
  },
  data() {
    return {
      themeOverrides: {
        common: {
          primaryColor: "#18a058",
        },
      },
    };
  },
});
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: "Microsoft YaHei", "PingFang SC", "Helvetica Neue", sans-serif;
  background-color: #f5f5f5;
}
</style>
