<script setup lang="ts">
import { OpenAPI } from "~/src/api";
const config = useRuntimeConfig();
OpenAPI.BASE = config.public.baseApiURL as string;
const { isMobile } = useDevice();
</script>

<script lang="ts">
export default {
  data() {
    return {};
  },
};
</script>

<template>
  <div>
    <UVerticalNavigation v-if="isMobile" :links="links" />
    <UHorizontalNavigation v-else :links="links"></UHorizontalNavigation>
    <NuxtPage></NuxtPage>
  </div>
</template>
