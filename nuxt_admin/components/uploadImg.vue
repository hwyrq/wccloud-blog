<template>
  <el-upload ref="uploadRef"  v-model:file-list="fileList"
             :limit="1"
             :action="url"
             list-type="picture-card"
             :headers="headers"
             :on-success="handleAvatarSuccess"
             :before-upload="beforeAvatarUpload"
             :class="fileList.length==1?'hide_box':''"
  >
    <el-icon >
      <Plus/>
    </el-icon>

    <template #file="{ file }">
      <div>
        <img class="el-upload-list__item-thumbnail" :src="file.url" alt=""/>
        <span class="el-upload-list__item-actions">
          <span
              class="el-upload-list__item-preview"
              @click="handlePictureCardPreview(file)"
          >
            <el-icon><zoom-in/></el-icon>
          </span>
          <span
              v-if="!disabled"
              class="el-upload-list__item-delete"
              @click="handleRemove(file)"
          >
            <el-icon><Delete/></el-icon>
          </span>
        </span>
      </div>
    </template>
  </el-upload>

  <el-dialog v-model="dialogVisible">
    <img w-full :src="dialogImageUrl" alt="Preview Image"/>
  </el-dialog>
</template>

<script lang="ts" setup>
import {Delete, Plus, ZoomIn} from '@element-plus/icons-vue'

import type {UploadFile, UploadProps, UploadUserFile} from 'element-plus'
const fileList = ref<UploadUserFile[]>([])
const imageUrl = ref('')
const dialogImageUrl = ref('')
const dialogVisible = ref(false)
const disabled = ref(false)
const headers = ref({Token: null});
const url = request.defaults.baseURL + "/wccloud-web-rust/file/upload";
const resultUrl = ref(null);
const uploadRef = ref(null);
const handleRemove = (file: UploadFile) => {
  fileList.value = [];
  resultUrl.value = null;

}

const handlePictureCardPreview = (file: UploadFile) => {
  dialogImageUrl.value = file.url!
  dialogVisible.value = true
}

const handleAvatarSuccess: UploadProps['onSuccess'] = (response, uploadFile) => {
  imageUrl.value = URL.createObjectURL(uploadFile.raw!);
  if (response.data != null) {
    resultUrl.value = response.data[0];
    ElMessage.success('上传成功');
  }
}

const beforeAvatarUpload: UploadProps['beforeUpload'] = (rawFile) => {
  headers.value.Token = localStorage.getItem("accessToken")
  if (rawFile.size / 1024 / 1024 > 5) {
    ElMessage.error('图片大小不能超过 5MB!')
    return false
  }
  ElMessage.info('上传中...');
  return true
}

defineExpose({
  url:resultUrl,
  fileList:fileList
});

</script>

<style  scoped>

.hide_box >>> .el-upload--picture-card{
  display: none;
}
</style>