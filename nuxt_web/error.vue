<template>
  <el-result
      icon="warning"
      title="页面不存在"
      sub-title="当前页面已离家出走，看看别的吧"
  >
    <template #extra>
      <el-button type="primary" @click="bk">返回主页</el-button>
    </template>
  </el-result>
</template>

<script setup lang="ts">
/**
 * @author wcz
 */

const bk = function () {
  useRouter().push("/");
};
</script>

<style scoped lang="css">

</style>