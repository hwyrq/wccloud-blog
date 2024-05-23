<template>
  <el-menu

      mode="horizontal"
      @select="handleSelect"
      style="justify-content: center"
  >
    <el-menu-item   >
      <a href="/" style="font-size: 22px;color: #006cff;font-weight: bold">{{ siteInfo.name }}</a>
    </el-menu-item>
    <el-menu-item v-for="item in web_navbar"  >
      <a :href="item.url">{{ item.name }}</a>
    </el-menu-item>
  </el-menu>

</template>

<script lang="ts" setup>
import {ref} from 'vue'
// import {getWebNavbar} from "~/api";
import {useAsyncData, useState} from "#app";
const handleSelect = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const instance = getCurrentInstance();
const siteInfo = instance?.attrs.siteInfo;


// const {data:data} = await useAsyncData(() => getWebNavbar({}));
// const web_navbar = data.value.data;
const web_navbar = ref([]);


onMounted( () => {

});

</script>
<style scoped lang="css">
a{
  text-decoration: none;
}
</style>
