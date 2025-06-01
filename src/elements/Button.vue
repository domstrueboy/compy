<template>
  <button :class="['button', type]" v-bind="$attrs"><slot /></button>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';

defineProps<{
  type?: 'accent' | 'danger' | 'default';
}>();
</script>

<style scoped>
.button {
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #dddddd;
  color: #333333;
  border: none;
  border-radius: 10px;
}
.accent {
  background-color: lightgreen;
}
.danger {
  background-color: lightcoral;
}
</style>
