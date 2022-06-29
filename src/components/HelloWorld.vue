<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'

defineProps({
  msg: String
})

const count = ref(0)
const msg1 = ref('')
onMounted(async () => {
  if (window.__TAURI_IPC__ !== undefined) {
    msg1.value = await invoke('hello')
  }
})
</script>

<template>
  <h1>{{ msg }}</h1>
  <h1>{{ msg1 }}</h1>
</template>

<style scoped>
a {
  color: #42b983;
}
</style>
