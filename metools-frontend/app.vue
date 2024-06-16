<script setup lang="ts">
import {OpenAPI} from "~/src/api";

OpenAPI.BASE = "https://metools.dudude-bit.ru"
const { isMobile } = useDevice();

const links = [
  [],
  [{
    "label": "Signup",
    "to": "/signup",
  }]
]
</script>

<template>
  <div>
    <UVerticalNavigation v-if="isMobile" :links="links"/>
    <UHorizontalNavigation v-else :links="links" ></UHorizontalNavigation>
    <NuxtPage></NuxtPage>
  </div>
</template>
