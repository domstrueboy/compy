<script setup lang="ts">
import Button from '../elements/Button.vue';
import { type IButton, EButtonType } from './Keyboard.d';

const values = [1, 2, 3, 4, 5, 6, 7, 8, 9, '<', 0, '='];
const buttons: IButton[] = values.map(value => ({
  type: value === '<' ? EButtonType.Clear : value === '=' ? EButtonType.Enter : EButtonType.Digit,
  text: value.toString(),
  value,
}));

const emit = defineEmits(['button-clicked']);
</script>

<template>
  <div class="keyboard">
    <Button
      v-for="button in buttons"
      :key="button.value"
      :accent="button.type !== EButtonType.Digit"
      @click="emit('button-clicked', button)"
    >
      {{ button.text }}
    </Button>
  </div>
</template>

<style scoped>
.keyboard {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  aspect-ratio: 3 / 4;
  gap: 10px;
}
</style>
