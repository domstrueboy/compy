<script setup lang="ts">
import { ref, computed } from 'vue';
import Keyboard from './components/Keyboard.vue';
import { type IButton, EButtonType } from './components/Keyboard.d';
import { EOperation } from './App.d';

const getRandomInt = (min: number = 1, max: number = 9) => {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

const a = ref(getRandomInt());
const b = ref(getRandomInt());
const operation = ref(EOperation.Multiply);

const correctAnswer = computed(() => {
  switch (operation.value) {
    case EOperation.Add:
      return a.value + b.value;
    case EOperation.Subtract:
      return a.value - b.value;
    case EOperation.Multiply:
      return a.value * b.value;
    case EOperation.Divide:
      return a.value / b.value;
  }
});

const answer = ref('');

const checkAnswer = () => {
  if (parseInt(answer.value) === correctAnswer.value) {
    alert('Правильно!');
    a.value = getRandomInt();
    b.value = getRandomInt();
    operation.value = EOperation.Multiply;
  } else {
    alert('Неправильно!');
  }
  answer.value = '';
}

const handleButtonClick = (button: IButton) => {
  if (button.type === EButtonType.Digit) {
    answer.value += button.value;
  } else if (button.type === EButtonType.Clear) {
    answer.value = answer.value.slice(0, -1);
  } else if (button.type === EButtonType.Enter) {
    checkAnswer();
  }
}
</script>

<template>
  <div class="container">
    <div class="equation">
      {{ a }} {{ operation }} {{ b }} = {{ answer }}
    </div>
    <Keyboard @buttonClicked = "handleButtonClick" />
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
