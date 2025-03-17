<template>
  <div class="home card">
    <img class="home-bg" src="@/assets/images/welcome.png" alt="welcome" />
    <h1 class="home-title">欢迎使用</h1>
    <slot></slot>
  </div>
</template>

<script setup lang="ts" name="home"></script>

<style scoped lang="scss">
@import "./index";
</style>
