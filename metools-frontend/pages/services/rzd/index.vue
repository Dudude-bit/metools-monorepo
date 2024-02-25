<template>
  <div>
    First name: {{ store.FirstName }}
    Last name: {{ store.LastName }}
  </div>
  <input type="text" @input="e => store.setFirstName(e.target.value)" placeholder="First name">
  <input type="text" @input="e => store.setLastName(e.target.value)" placeholder="Last name">
</template>

<script setup lang="ts">

import { useUserStore } from "@/stores/user";

const store = useUserStore()


</script>