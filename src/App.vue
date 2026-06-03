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
          primaryColor: "#60A5FA",
          primaryColorHover: "#3B82F6",
          primaryColorPressed: "#2563EB",
          primaryColorSuppl: "#60A5FA",
          borderRadius: "8px",
          borderRadiusSmall: "6px",
          fontFamily: "'DM Sans', 'Microsoft YaHei', sans-serif",
        },
        Button: {
          borderRadiusMedium: "8px",
          borderRadiusSmall: "6px",
        },
        Card: {
          borderRadius: "12px",
          paddingMedium: "20px",
          paddingSmall: "16px",
        },
      },
    };
  },
});
</script>

<style>
@import url("https://fonts.googleapis.com/css2?family=DM+Sans:ital,opsz,wght@0,9..40,300;0,9..40,400;0,9..40,500;0,9..40,600;0,9..40,700;1,9..40,400&family=Fraunces:ital,opsz,wght@0,9..144,400;0,9..144,600;0,9..144,700;1,9..144,400&display=swap");

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: "DM Sans", "Microsoft YaHei", "PingFang SC", sans-serif;
  background-color: #F0F7FF;
  color: #1E3A5F;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  overflow: hidden;
}

::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #BFDBFE;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #93C5FD;
}
</style>