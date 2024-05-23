<template>
    <el-space direction="vertical" size="default"   >
      <el-card v-for="item in pageList" class="box-card" style="width: 800px;padding-bottom: 20px;" shadow="hover">
        <h3 >
          <a :href="'/info/'+item.blogId" target="_self" >{{ item.title }}</a>
        </h3>
        <span >
          <a :href="'/info/'+item.blogId" target="_self"  title>
            <img style="width: 250px" v-if="item.img" :src="item.img" alt>
          </a>
        </span>
        <p  style="font-size: 14px;color: #566573">{{ item.summary }}</p>
      </el-card>
      <div style="margin-bottom: 10px">
        <el-button type="primary" round @click="loadContent" v-if="!isEnd">查看更多</el-button>
        <el-button type="info" round  v-if="isEnd">没有更多了</el-button>
      </div>
    </el-space>

</template>

<script type="ts" setup>
import {page} from "~/api";
import {useAsyncData} from "#app";
const isEnd = ref(false);
const total = ref();
let pageSize = 2;
let pageNum = 1;
// let pageList = ref([]);
const {data:{value:data}} = await useAsyncData(() => page({pageNum: pageNum, pageSize: pageSize}));
let pageList = ref(data.data.list);
total.value = data.data.total;

onMounted( () => {
   loadContent();
});

const loadContent = async function () {
  pageNum = pageNum + 1;
   await page({pageNum: pageNum, pageSize: pageSize}).then(data => {
    if (data.data.list.length > 0) {
      isEnd.value = false;
      const newData = pageList.value.concat(data.data.list);
      pageList.value = newData;
      total.value = data.data.total;
      if (newData.length < pageSize) {
        isEnd.value = true;
      }
    } else {
      isEnd.value = true;
    }
  });
}


</script>

<style scoped lang="css">
li {
  float: left;
  margin-left: 30px;
  list-style-type: none;
  color: #748594;
}
a{
  text-decoration: none;
  color: #555;
  font-size: 20px;
}
a:hover{
  color: #337ab7;
}
li a{
  color: #748594 !important;
  font-size: 12px !important;
}
li a:hover{
  color: #337ab7 !important;
}


li span{
  margin-right: 3px;
}
</style>
