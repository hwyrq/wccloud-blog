<template>
  <template v-for="(item) in getCurrentInstance()?.attrs.listMenu">

    <el-menu-item :index="item.menuId" v-if="item!=null&&item.type==2" >
      <template #title>{{ item.menuName }}</template>
    </el-menu-item>
    <el-sub-menu :index="item.menuId" v-if="item!=null&&item.type==1">
      <template #title>{{ item.menuName }}</template>
      <menu-tree v-if="item.children!=null&&item.children.length!=0" :listMenu="item.children"></menu-tree>
    </el-sub-menu>
  </template>
</template>
<script setup lang="ts">
/**
 * @author wcz
 */

onMounted(() => {

});

</script>
<style scoped lang="css">

</style>
