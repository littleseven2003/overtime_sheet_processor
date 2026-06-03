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
          primaryColor: "#2563eb",
          primaryColorHover: "#1e3a8a",
          primaryColorPressed: "#1e3a8a",
          primaryColorSuppl: "#2563eb",
          borderRadius: "10px",
          borderRadiusSmall: "8px",
          fontFamily: "'DM Sans', 'Microsoft YaHei', sans-serif",
        },
        Button: {
          borderRadiusMedium: "10px",
          borderRadiusSmall: "8px",
        },
        Card: {
          borderRadius: "16px",
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

:root {
  --color-brand: #2563eb;
  --color-brand-strong: #1e3a8a;
  --color-brand-soft: #dbeafe;
  --color-accent: #0891b2;
  --color-accent-soft: #cffafe;
  --color-success: #0891b2;
  --color-success-bg: #cffafe;
  --color-ink: #172033;
  --color-muted: #596579;
  --color-faint: #8792a5;
  --color-line: #d6e0ef;
  --color-panel: #ffffff;
  --color-panel-soft: #f7fbff;
  --color-page: #eff6ff;
  --shadow-panel: 0 18px 60px rgba(30, 64, 175, 0.11);
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: "DM Sans", "Microsoft YaHei", "PingFang SC", sans-serif;
  background: var(--color-page);
  color: var(--color-ink);
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
  background: var(--color-line);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: var(--color-faint);
}
</style>
