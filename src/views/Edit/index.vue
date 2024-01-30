<style lang="scss" scoped>
@import "https://gw.alipayobjects.com/render/p/yuyan_npm/@alipay_lakex-doc/1.12.0/umd/doc.css";
@import "https://unpkg.com/antd@4.24.13/dist/antd.css";
</style>

<template>
  <div id="editor" class="container w-full h-full"></div>
</template>

<script setup lang="ts">
import { useFileStore } from "@/store/fileStore";
import { nextTick, onMounted, watch } from "vue";
import { storeToRefs } from 'pinia'

const fileStore = useFileStore()

const { activePath, contentCache } = storeToRefs(fileStore);

let editor: any = null

watch(() => activePath.value, () => {
  console.log("[activePath]: ", activePath.value)
  editor.setDocument("text/markdown", contentCache.value[activePath.value])
})

const useEditor = () => {
  let times = 0;

  const init = function () {
    return Promise.all([
      // @ts-ignore
      import("@/assets/editor/react.min.js"),
      // @ts-ignore
      import("@/assets/editor/react-dom.min.js"),
      // @ts-ignore
      import("@/assets/editor/yuque.min.js"),
    ])
      .then((res) => {
        const { createOpenEditor } = (window as any).Doc || {};

        console.log(window, res)

        if (createOpenEditor) {
          // 创建编辑器
          editor = createOpenEditor(document.getElementById("editor"), {
            input: {},
            image: {
              isCaptureImageURL() {
                return false;
              },
            },
          });
          // 设置内容
          editor.setDocument("text/markdown", "欢迎来到yuque编辑器");
          // 监听内容变动
          editor.on("contentchange", () => {
            console.info(editor.getDocument("text/markdown"));
          });
        }
      })
      .catch((err) => {
        console.log("[init]: ", err);
        retry();
      });
  };

  const retry = async () => {
    times += 1;

    if (times > 10) {
      return;
    }

    try {
      await init();
    } catch (e) {
      retry();
    }
  };

  return {
    init,
    retry,
  };
};



onMounted(() => {
  nextTick(() => {
    const { init } = useEditor();
    init();
  });
});
</script>
