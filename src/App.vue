<template>
  <router-view />
</template>

<script setup lang="ts">
import { useColorMode } from "@vueuse/core";
useColorMode() || "dark";
</script>
