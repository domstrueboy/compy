<script setup lang="ts">
import { ref } from 'vue';

const getRandomInt = (min: number = 1, max: number = 9) => {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

const a = ref(getRandomInt());
const b = ref(getRandomInt());
const result = ref(a.value * b.value);
const answer = ref('');

const checkAnswer = () => {
  if (parseInt(answer.value) === result.value) {
    alert('Правильно!');
  } else {
    alert('Неправильно!');
  }
}
</script>

<template>
  {{ a }} х {{ b }} = <input v-model="answer" /> <button @click="checkAnswer">-></button>
</template>

<style scoped>

</style>
