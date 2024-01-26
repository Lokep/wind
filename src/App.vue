<style scoped lang="scss">

@keyframes van-fade-in {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}

@keyframes van-fade-out {
  from {
    opacity: 1;
  }

  to {
    opacity: 0;
  }
}

.fade {
  &-enter-active {
    animation: 0.3s van-fade-in both ease-out;
  }

  &-leave-active {
    animation: 0.3s van-fade-out both ease-in;
  }
}
</style>

<template>
  <router-view v-slot="{ Component }">
    <keep-alive>
      <transition name="fade">
        <component :is="Component" />
      </transition>
    </keep-alive>
  </router-view>
</template>

<script setup lang="ts"></script>
