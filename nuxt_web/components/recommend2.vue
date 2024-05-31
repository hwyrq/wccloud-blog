
<template>
  <el-space  direction="vertical">
    <el-card  shadow="hover" style="width: 400px;height: 220px;" v-for="item in dataList" >
      <a :href="'/info/'+item.blogId" target="_self">
        <img
            style="width: 400px;height: 220px;display: block"
            v-if="item.imgUrl"
            :src="item.imgUrl"

        />
        <div style="width: 400px;height: 220px;background-color: rgb(248,248,248)" v-if="!item.imgUrl"></div>

        <div  class="text-span">
          <span>{{ item.title }}</span>
        </div>
      </a>

    </el-card>
  </el-space>

</template>
<script setup lang="ts">
import {useAsyncData} from "#app";
import {level} from "~/api";

let {data:{value:data}} = await useAsyncData(() => level({level: 2}));
const dataList = data.data;
</script>

<style scoped>
>>>.el-card__body {
  padding: 0;
  position: absolute;
}

.text-span{
  position: absolute;
  text-align: center;
  width: 400px;
  bottom: 20px;
  /* height: 40px; */
  /* border: 1px solid grey; */
  background: rgba(0, 0, 0, 0.3);
  font-size: 15px;
  color: white;
  line-height: 25px;
}
</style>
