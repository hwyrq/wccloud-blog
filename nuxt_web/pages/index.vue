<template>
    <el-space direction="vertical" style="margin-top: 10px;margin-right: 10px">
      <recommend1/>
      <BlogList/>
    </el-space>
  <el-space style="margin-top: 10px;margin-bottom: 10px" direction="vertical" >
      <recommend2/>
      <label-card/>
      <friendly-link/>
  </el-space>
</template>

<script setup lang="ts">

import Recommend1 from "~/components/recommend1.vue";
import Recommend2 from "~/components/recommend2.vue";
import BlogList from "~/components/blogList.vue";

const instance = getCurrentInstance();
const siteInfo = instance?.attrs.siteInfo;
useHead({
  title: siteInfo.title,
  meta:[
    {name:'description',content:siteInfo.summary},
    {name:'keyword',content:siteInfo.keyword},
  ]
});
</script>

<style scoped lang="css">

</style>
