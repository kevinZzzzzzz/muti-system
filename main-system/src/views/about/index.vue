<template>
  <DataVisualizeComp />
</template>

<script setup lang="ts" name="about">
import { defineAsyncComponent } from "vue";

const { pkg, lastBuildTime } = __APP_INFO__;
const { dependencies, devDependencies, version } = pkg;
const DataVisualizeComp = defineAsyncComponent(() => import("remote_main2/DataVisualize"));
</script>

<style lang="scss" scoped>
.card {
  .title {
    margin: 0 0 15px;
    font-size: 17px;
    font-weight: bold;
    color: var(--el-text-color-primary);
  }
  .text {
    font-size: 15px;
    line-height: 25px;
    color: var(--el-text-color-regular);
    .el-link {
      font-size: 15px;
    }
  }
}
</style>
