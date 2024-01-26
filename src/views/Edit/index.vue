<style lang="scss" scoped>
@import 'https://gw.alipayobjects.com/render/p/yuyan_npm/@alipay_lakex-doc/1.12.0/umd/doc.css';
@import 'https://unpkg.com/antd@4.24.13/dist/antd.css';
</style>

<template>
  <div id="editor" class="container"></div>
</template>

<script setup lang="ts">
import { onMounted } from "vue";

const useEditor = () => {
  let times = 0;

  const init = function () {
    return Promise.all([
      // @ts-ignore
      import("https://unpkg.com/react@18/umd/react.production.min.js"),
      // @ts-ignore
      import("https://unpkg.com/react-dom@18/umd/react-dom.production.min.js"),
      // @ts-ignore
      import(
        "https://gw.alipayobjects.com/render/p/yuyan_npm/@alipay_lakex-doc/1.12.0/umd/doc.umd.js"
      ),
    ])
      .then(() => {
        const { createOpenEditor } = (window as any).Doc;
        // 创建编辑器
        const editor = createOpenEditor(document.getElementById("editor"), {
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

const { init, retry } = useEditor();

onMounted(() => {
  init();
});
</script>
