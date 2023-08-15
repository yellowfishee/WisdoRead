<script setup lang="ts">
import {
  NButton, NLayout, NSpace, NLayoutFooter, NLayoutHeader,
  NLayoutContent, NUpload, NUploadDragger, NIcon, NText, NP,
  NCard, NEmpty, NMessageProvider
} from 'naive-ui'
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'
import { reactive, } from 'vue'
import { copyFile, BaseDirectory } from '@tauri-apps/api/fs';
import { sendNotification } from '@tauri-apps/api/notification';
import { invoke } from '@tauri-apps/api/tauri';


components: {
  ArchiveIcon
}
// 模拟数据
const books = reactive([
  {
    id: 1,
    title: '《JavaScript高级程序设计》',
  }
])



async function getBookOrigin(file) {
  console.log(file.file.file instanceof File);
  // const result = await invoke('save_file', {
  //   name: file.file.name,
  //   bytes: file.file.file
  // })
  // const message = useMessage();
  // if (result == 200) {
  //   message.success('上传成功')
  // } else {
  //   message.error('上传失败')
  // }
  console.log(BaseDirectory);
  const app_dir = await invoke('app_dir');
  console.log(app_dir);

  await copyFile(file.file.name, file.file.name, { dir: BaseDirectory.Resource }).then((res) => {
    console.log(res);

    sendNotification('Tauri is awesome!');
    sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' });

  })

}


</script>

<template>
  <n-space vertical>
    <n-layout>
      <n-layout-header>首页</n-layout-header>
      <n-layout-content content-style="padding: 24px;">
        <n-card v-for="book in books" :key="book.id" :title="book.title">
          <template #cover>
            <img src="https://07akioni.oss-cn-beijing.aliyuncs.com/07akioni.jpeg">
          </template>
          卡片内容
        </n-card>
        <n-empty description="你什么也找不到" v-if="books === null">
          <template #extra>
            <n-button size="small">
              看看别的
            </n-button>
          </template>
        </n-empty>
      </n-layout-content>
      <n-layout-footer>
        <n-message-provider>
          <n-upload multiple directory-dnd :max="5" :on-before-upload=getBookOrigin>
            <n-upload-dragger>
              <div style="margin-bottom: 12px">
                <n-icon size="48" :depth="3">
                  <archive-icon />
                </n-icon>
              </div>
              <n-text style="font-size: 16px">
                点击或者拖动文件到该区域来上传
              </n-text>
              <n-p depth="3" style="margin: 8px 0 0 0">
                请不要上传敏感数据，比如你的银行卡号和密码，信用卡号有效期和安全码
              </n-p>
            </n-upload-dragger>
          </n-upload>
        </n-message-provider>
      </n-layout-footer>
    </n-layout>
  </n-space>
</template>

<style scoped>
.n-card {
  max-width: 300px;
}
</style>
