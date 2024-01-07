<template>
  <div>
  <h1>Hello, {{ name }}</h1>
</div>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api';
import { onMounted, ref } from 'vue';

const name = ref();

onMounted(async () => {
  name.value = await invoke("get_name") ;

});

</script>

<style>

</style>

