<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import Keyboard from './components/Keyboard.vue';
import { type IButton, EButtonType } from './components/Keyboard.d';
import { EOperation } from './App.d';

const ANSWER_DELAY = 1000;

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
const isCorrect = ref<boolean | null>(null);

const streak = ref(0);
const maxStreak = ref(Number(localStorage.getItem('maxStreak')) || 0);
watch(streak, () => {
  if (streak.value <= maxStreak.value) return;
  maxStreak.value = streak.value;
  localStorage.setItem('maxStreak', maxStreak.value.toString());
});

const checkAnswer = async () => {
  isCorrect.value = parseInt(answer.value) === correctAnswer.value;
  if (isCorrect.value) {
    streak.value++;
    await new Promise(resolve => setTimeout(resolve, ANSWER_DELAY));
    a.value = getRandomInt();
    b.value = getRandomInt();
  } else {
    streak.value = 0;
    await new Promise(resolve => setTimeout(resolve, ANSWER_DELAY));
  }
  answer.value = '';
  isCorrect.value = null;
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

    <div :class="['equation', { correct: isCorrect === true, error: isCorrect === false }]">
      {{ a }} {{ operation }} {{ b }} = {{ answer }}
    </div>

    <div class="message">
      <div v-if="isCorrect === true" class="correct">Правильно!</div>
      <div v-else-if="isCorrect === false" class="error">Неправильно! Попробуй ещё раз!</div>
    </div>

    <Keyboard @buttonClicked = "handleButtonClick" />

    <div class="streak-wrapper">
      <div title="Правильных ответов подряд" class="streak">
        <span>🔥</span>
        <span>{{ streak }}</span>
        <span class="streak-title">подряд</span>
      </div>
      
      <div title="Рекорд" class="streak">
        <span>🏅</span>
        <span>{{ maxStreak }}</span>
        <span class="streak-title">рекорд</span>
      </div>
    </div>

  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  .equation {
    font-size: 64px;

    &.correct {
      color: lightgreen;
    }
    &.error {
      color: red;
    }
  }

  .message {
    height: 16px;
    font-size: 16px;
    margin-bottom: 16px;

    .correct {
      color: lightgreen;
    }
    .error {
      color: red;
    }
  }

  .streak-wrapper {
    margin-top: 24px;
    display: flex;
    justify-content: space-between;
    gap: 16px;
    width: 100%;
    font-size: 48px;

    .streak {
      display: flex;
      flex-wrap: nowrap;
      align-items: flex-end;

      .streak-title {
        font-size: 16px;
        margin-left: 4px;
        margin-bottom: 4px;
      }
    }
  }
}
</style>
