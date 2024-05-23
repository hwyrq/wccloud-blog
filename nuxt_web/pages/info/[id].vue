<template>
  <el-space style="align-items: normal">
    <el-card style="width: 800px;margin-top: 10px;margin-bottom: 10px">
      <h1 v-if="blogData.title">{{ blogData.title }}</h1>
      <div v-html="blogData.html"></div>
    </el-card>
  </el-space>
</template>

<script setup lang="ts">

import {useAsyncData} from "#app";
import {one} from "~/api";

const route = useRoute();
const {data:{value:data}} = await useAsyncData(() => one({blogId: route.params.id}));
const blogData = data.data;

const instance = getCurrentInstance();
const siteInfo = instance?.attrs.siteInfo;

useHead({
  title: blogData.title + " - " + siteInfo.title,
  meta: [
    {name: 'description',content: blogData.summary},
    {name: 'keyword',content: blogData.title}
  ]
});

</script>

<style>

</style>
