<style lang="scss" scoped>
.layout {
  .leftbar {
    width: 30vw;
    max-width: 300px;

    &-title {
      text-align: center;
      padding: 32px 24px;
      font-size: 32px;
    }
  }

  .menu {
    &-item {
      padding: 8px 12px;
      border-radius: 5px;
      cursor: pointer;
      transition: all 0.3s;
      margin: 2px 8px;
      &:hover {
        background: #eee;
      }
    }
  }
}
</style>

<template>
  <div class="layout w-full h-screen flex gap-5">
    <div class="leftbar shrink-0">
      <div class="leftbar-title">leftbar</div>

      <div class="menu">
        <div
          class="menu-item flex items-center justify-between"
          v-for="item in directories"
          :key="item.name"
          @click="() => onClickMenuItem(item)"
        >
          <div class="flex-1">{{ item.name }}</div>
          <div
            class="shrink-0 iconfont icondirection-right"
            v-if="item.is_dir"
          ></div>
        </div>
      </div>
    </div>
    <div class="flex-1 bg-slate-50">
      <RouterView></RouterView>
    </div>
  </div>
</template>

<script setup lang="ts">
import { storeToRefs } from "pinia";
import { useFileStore } from "@/store/fileStore";

const fileStore = useFileStore();

const { directories } = storeToRefs(fileStore);

fileStore.getDirectories();

const onClickMenuItem = ({path, is_file}: any) => {
   fileStore.getFileContent(path, is_file)
}
</script>
