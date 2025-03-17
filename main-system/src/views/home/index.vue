<template>
  <div class="home card">
    <!-- <img class="home-bg" src="@/assets/images/welcome.png" alt="welcome" /> -->
    <HomePageComp>
      <p>hello world</p>
    </HomePageComp>
    <UseProTablePageComp />
  </div>
</template>

<script setup lang="ts" name="home">
import { defineAsyncComponent } from "vue";
// const useProTablePageComp = defineAsyncComponent(() => );
// import UseProTablePageComp from "remote_main1/UseProTablePage";
const HomePageComp = defineAsyncComponent(() => import("remote_main1/HomePage"));
const UseProTablePageComp = defineAsyncComponent(() => import("remote_main1/UseProTablePage"));
</script>

<style scoped lang="scss">
@import "./index";
</style>
