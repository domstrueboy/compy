<template>
  <button class="button" v-bind="$attrs"><slot /></button>
</template>

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
</style>
