<template>
  <div :class="[
    'SysNav',
    judVarIsBoolean(isOpen) && isOpen && 'SysNav-isOpen',
    judVarIsBoolean(isOpen) && !isOpen && 'SysNav-isClose'
  ]">
    <div class="SysNav_main">
      <ul class="SysNav_main_list">
        <li v-for="item in menuList" :key="item.key" class="SysNav_main_list_item" @click="goToUrl(item.url)">
          <img src="@/assets/images/menu-s.png" alt="" />
          <p style="color: #000000">{{ item.name }}</p>
        </li>
      </ul>
    </div>
  </div>
  <div class='SysNavIcon' @click="handleNavOpen()">
    <img v-if="isOpen" src="@/assets/images/closeIcon.png" alt="" />
    <img v-else src="@/assets/images/menuIcon.png" alt="" />
  </div>
</template>

<script setup lang="ts" name="SysNavComp">
import { ref } from "vue";
import { menuList } from "./mock";

const isOpen = ref<boolean | null>(null); // 是否展开 null是初始状态
const judVarIsBoolean = val => {
  return typeof val === "boolean";
};
const handleNavOpen = () => {
  // 展开收起
  isOpen.value = !isOpen.value;
};
const goToUrl = (url: string) => {
  if (url) {
    window.location.href = url;
  }
};
</script>

<style scoped lang="scss">
@import "@/styles/reset";
@import "./index";
</style>
