<template>
  <h1 style="display: none">{{siteInfo.summary}}</h1>
  <el-container style="min-height: 1200px;background-color: #eeeeee;align-self: center;">
<!--    <el-affix>-->
      <el-header style="padding: 0">
        <Header :siteInfo="siteInfo"/>
      </el-header>
<!--    </el-affix>-->

    <el-container style="background-color: #eeeeee;align-self: center;">
      <nuxt-layout >
        <NuxtPage :siteInfo="siteInfo" />
      </nuxt-layout>
    </el-container>
    <el-footer>
      Copyright <a href="http://wccloud.top/"> {{ siteInfo.name }}</a>
      <a href="https://beian.miit.gov.cn/" target="_blank">{{ siteInfo.recordNum }}</a>
      <a href="https://beian.mps.gov.cn/#/query/webSearch?code=34019202002289" rel="noreferrer" target="_blank">皖公网安备34019202002289</a>
    </el-footer>
  </el-container>
</template>
<script setup lang="ts">
import Header from "~/components/header.vue";
import {useAsyncData} from "#app";

const style = ref({height: "1200px"});

const siteInfo = ref({
  name: "wccloud",
  title: "wccloud",
  summary: "一个专注于技术分享的博客平台，大家以共同学习，乐于分享，拥抱开源的价值观进行学习交流",
  keyword: "IT社区,技术社区,Java技术分享,Spring教程,开发者社区,Rust",
  author: "MR.WANG",
  recordNum: "皖ICP备2024037398号-1",
  email: "hwyrqw@163.com",
});
onMounted(() => {
  /*window.addEventListener("resize", () => {
    style.value.height = window.innerHeight - 30 + "px";
  });*/
});
// const {data: data} = await useAsyncData(() => getWebConfig());
// const siteInfo = data.value.data;

useHead({
/*  title: siteInfo.title,
  meta:[
    {name:'description',content:siteInfo.summary},
    {name:'keyword',content:siteInfo.keyword},
  ],*/
  htmlAttrs:{
    lang:'zh'
  }
});

</script>
<style scoped lang="css">
.el-footer {
  color: #a5a4a4;
  text-align: center;
  line-height: 30px;
  height: 30px;
  font-size: 10px;
}

.el-footer a {
  margin-left: 5px;
  text-decoration: none;
  color: #a5a4a4;
}

.el-footer a:hover {
  margin-left: 5px;
  text-decoration: none;
  color: #2fc2d0;
}


.el-aside {
  background-color: #D3DCE6;
  color: #333;
  text-align: center;
  line-height: 225px;
}

.el-main {
  background-color: #E9EEF3;
  color: #333;
  text-align: center;
  line-height: 160px;
}


</style>
