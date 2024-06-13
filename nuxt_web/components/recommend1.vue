
<template>
    <el-card style="width: 800px;" shadow="hover">
      <el-carousel style="width: 800px;height: 450px;" autoplay :interval="2000" :motion-blur="true" :pause-on-hover="pause_on_hover">
        <el-carousel-item style="width: 800px;height: 450px;cursor: pointer" v-for="(item,index) in dataList" :key="index" >
          <a :href="'/info/'+item.blogId" target="_self">
            <img
                style="width: 800px;height: 450px"
                v-if="item.imgUrl"
                :src="item.imgUrl"

                />
            <div style="width: 800px;height: 450px;background-color: rgb(248,248,248)" v-if="!item.imgUrl"></div>
            <div  class="text-span">
              <span>{{ item.title }}</span>
            </div>
          </a>

        </el-carousel-item>
      </el-carousel>
    </el-card>

</template>

<script setup lang="ts">
import {useAsyncData} from "#app";
import {level} from "~/api";

const pause_on_hover = ref(false);
const dataList = ref();//[{title: ''}],
 useAsyncData(() => level({level: 1}).then(response => {
   dataList.value = response.data;

}));
 onMounted(()=>{
   setTimeout(() => {
     //解决首次加载页面后不会自动轮播的问题，这样既能保证轮播正常，也能保证悬停鼠标暂停轮播
     pause_on_hover.value = true;
   }, 1000);

 })

</script>

<style scoped lang="css">
>>>.el-card__body {
  padding: 0;
}
.text-span{
  position: absolute;
  text-align: center;
  width: 800px;
  bottom: 40px;
   height: 40px;
  /* border: 1px solid grey; */
  background: rgba(0, 0, 0, 0.3);
  font-size: 22px;
  color: white;
  line-height: 40px;
  overflow: hidden;
}
</style>
