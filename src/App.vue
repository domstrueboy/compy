<script setup lang="ts">
import { ref, computed } from 'vue';
import Keyboard from './components/Keyboard.vue';
const getRandomInt = (min: number = 1, max: number = 9) => {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

enum Operation {
  Add = '+',
  Subtract = '-',
  Multiply = 'x',
  Divide = ':',
}

const a = ref(getRandomInt());
const b = ref(getRandomInt());
const operation = ref(Operation.Multiply);
const correctAnswer = computed(() => {
  switch (operation.value) {
    case Operation.Add:
      return a.value + b.value;
    case Operation.Subtract:
      return a.value - b.value;
    case Operation.Multiply:
      return a.value * b.value;
    case Operation.Divide:
      return a.value / b.value;
  }
});

const answer = ref('');

const checkAnswer = () => {
  if (parseInt(answer.value) === correctAnswer.value) {
    alert('Правильно!');
  } else {
    alert('Неправильно!');
  }
}
</script>

<template>
  <div class="container">
    <div class="equation">
      {{ a }} {{ operation }} {{ b }} = {{ answer }}
    </div>
    <Keyboard />
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 36px;

  .equation {
    font-size: 64px;
  }
}
</style>
