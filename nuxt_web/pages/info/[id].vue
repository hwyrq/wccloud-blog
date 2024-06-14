<template>
  <el-space style="align-items: normal">
    <div  id="outline" style="position: fixed;top: 100px;left:200px"></div>
    <el-card style="width: 800px;margin-top: 10px;margin-bottom: 10px">
      <h1 v-if="blogData.title">{{ blogData.title }}</h1>
      <div style="width: 100%;border: 1px solid #666666"></div>
      <div id="preview" v-html="blogData.html"></div>
    </el-card>
  </el-space>
</template>

<script setup lang="ts">

import {useAsyncData} from "#app";
import {one} from "~/api";
import 'vditor/dist/index.css'
import Vditor from "vditor";

const route = useRoute();
const {data:{value:data}} = await useAsyncData(() => one({blogId: route.params.id}));
const blogData = data.data;


onMounted(async () => {

  const outlineElement = document.getElementById('outline');
  const previewElement = document.getElementById('preview');
  Vditor.preview(previewElement, blogData.md, {
    mode: "dark",
    hljs: {style: "dracula"},
    cdn: "/vditor",
    markdown: {toc: true,mark:true},
    speech: {
      enable: true,
    },
    anchor: 1, after() {
      Vditor.outlineRender(previewElement, outlineElement)
      if (outlineElement.innerText.trim() !== '') {
        outlineElement.style.display = 'block'
      }
    },

  });

  const outline_refresh = function () {
    if (window.innerWidth <= 1200) {
      outlineElement.style = "display: none;";
    }else{
      outlineElement.style = "display: block;position: fixed;top: 100px;left:"+(window.innerWidth/2-650)+"px";
    }
  };
  onresize = function () {
    outline_refresh();
  };
  outline_refresh();



})
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

<style lang="css" scoped>

</style>
