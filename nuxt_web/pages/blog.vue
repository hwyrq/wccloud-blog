<template>
  <div style="margin:15px">
    <el-button type="primary" plain @click="add">新增</el-button>
    <el-button type="primary" plain @click="refresh">刷新</el-button>
  </div>

  <el-table
      :data="pageData.list"
      style="width: 100%; margin-bottom: 20px"
      row-key="menuId"
      border stripe="stripe"
  >
    <el-table-column prop="title" label="标题" align="center" header-align="center"  show-overflow-tooltip />
    <el-table-column prop="summary" label="简介" align="center" header-align="center"  show-overflow-tooltip	/>
    <el-table-column prop="typeName" label="分类"  width="80" align="center" header-align="center"  show-overflow-tooltip/>
    <el-table-column prop="labelName" label="标签"   align="center" header-align="center"  show-overflow-tooltip>
      <template #default="scope">
        <el-tag v-for="item in scope.row.labelName.split(',')" type="primary" style="margin-left: 5px">{{item}}</el-tag>
      </template>
    </el-table-column>
    <el-table-column prop="level" label="推荐"  width="80" align="center" header-align="center"  show-overflow-tooltip/>
    <el-table-column prop="enableComment" label="评论"  width="80" align="center" header-align="center"  show-overflow-tooltip>
      <template #default="scope">
        <el-tag type="success" v-if="scope.row.enableComment==1">开启</el-tag>
        <el-tag type="danger" v-if="scope.row.enableComment!=1">关闭</el-tag>
      </template>
    </el-table-column>
    <el-table-column prop="status" label="状态"  width="80" align="center" header-align="center"  show-overflow-tooltip>
      <template #default="scope">
        <el-tag type="success" v-if="scope.row.status==0">发布</el-tag>
        <el-tag type="danger" v-if="scope.row.status==1">下架</el-tag>
      </template>
    </el-table-column>

    <el-table-column label="操作"  width="160" align="center" header-align="center"  show-overflow-tooltip>
      <template #default="scope">
        <el-button type="primary" :icon="Edit" circle plain @click="edit(scope.row)"/>
        <el-button type="danger" :icon="Delete" circle plain @click="deleteHandler(scope.row)"/>
      </template>
    </el-table-column>
  </el-table>
  <el-pagination  background  layout="->,total,sizes,prev, pager, next, jumper"
                   :page-size="pageData.pageSize" :total="pageData.total" :page-sizes="pageData.pageSizes"
                   v-model:current-page="pageData.currentPage" v-model:page-size="pageData.pageSize"
                   @change="refresh"
  />

</template>
<script setup lang="ts">
import {Delete, Edit} from "@element-plus/icons-vue";
import {page,del} from "~/api/blog";

const pageData = ref({list:[],total:0,pageSize:10,currentPage:1,pageSizes:[ 10, 50, 100]});
onMounted(async () => {
  await refresh();
});

const form = ref({});
const add = function () {
  form.value = {};
  // dialogTitle.value = "新增";
  // dialogVisible.value = true;
  useRouter().push("/edit");
};
const refresh = async function () {
  let data = (await page({pageNum: pageData.value.currentPage, pageSize: pageData.value.pageSize})).data;
  pageData.value.list = data.list;
  pageData.value.total = data.total;

};
const edit = function (t: any) {
  /*form.value = {};
  Object.assign(form.value, t);
  dialogTitle.value = "编辑";
  dialogVisible.value = true;*/
  useRouter().push("/edit?id="+t.blogId);
};

const deleteHandler = async function (t: any) {
  ElMessageBox.confirm('确认删除吗？', '删除').then(async () => {
    let data = await del({blogId:t.blogId});
    if (data.code == 0) {
      ElMessage({message: "删除成功", type: "success"});
      await refresh();
    } else {
      ElMessage({message: data.msg, type: "error"});
    }
  });

};

</script>
<style scoped lang="css">

</style>
