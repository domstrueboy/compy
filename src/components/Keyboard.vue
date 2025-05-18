<script setup lang="ts">
import Button from '../elements/Button.vue';

enum Type { Digit = 'digit', Clear = 'clear', Enter = 'enter' };

type Button = {
  type: Type;
  text: string;
  value: number | string;
}

const values = [1, 2, 3, 4, 5, 6, 7, 8, 9, '<', 0, '='];
const buttons: Button[] = values.map(value => ({
  type: value === '<' ? Type.Clear : value === '=' ? Type.Enter : Type.Digit,
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
      :accent="button.type !== Type.Digit"
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
