<template>
  <div class="SysNav">
    <div
      :class="[
        'SysNav_main',
        judVarIsBoolean(isOpen) && isOpen && 'SysNav_main-isOpen',
        judVarIsBoolean(isOpen) && !isOpen && 'SysNav_main-isClose'
      ]"
    >
      <ul class="SysNav_main_list">
        <li v-for="item in menuList" :key="item.key" class="SysNav_main_list_item" @click="goToUrl(item.url)">
          <img src="@/assets/images/system.png" alt="" />
          <p style="color: #000000">{{ item.name }}</p>
        </li>
      </ul>
      <div
        :class="[
          'SysNav_main_icon',
          judVarIsBoolean(isOpen) && isOpen && 'SysNav_main_icon-isOpen',
          judVarIsBoolean(isOpen) && !isOpen && 'SysNav_main_icon-isClose'
        ]"
        @click="handleNavOpen()"
      >
        <img v-if="isOpen" src="@/assets/images/turnUp.png" alt="" />
        <img v-else src="@/assets/images/turnDown.png" alt="" />
      </div>
    </div>
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
