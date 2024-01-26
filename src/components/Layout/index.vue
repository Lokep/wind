<style lang="scss" scoped>
.layout {
  .leftbar {
    width: 300px;
  }
}
</style>

<template>
  <div class="layout w-full h-screen flex gap-5">
    <div class="leftbar shrink-0">leftbar</div>
    <div class="flex-1">
      <RouterView></RouterView>
    </div>
  </div>
</template>

<script setup lang="ts"></script>
