

<template>
  <div style="margin:15px">
    <el-button type="primary" plain @click="add">新增</el-button>
    <el-button type="primary" plain @click="refresh">刷新</el-button>
  </div>

  <el-skeleton v-if="menuList.length==0" :rows="10" animated  />
  <el-table v-if="menuList.length!=0"
      :data="menuList"
      style="width: 100%; margin-bottom: 20px"
      row-key="menuId"
      border
  >
    <el-table-column prop="menuName" label="菜单"  />
    <el-table-column prop="path" label="路径"  />
    <el-table-column prop="type" label="类型"  >
      <template #default="scope">
        <span  v-if="scope.row.type==1">目录</span>
        <span  v-if="scope.row.type==2">菜单</span>
        <span  v-if="scope.row.type==3">按钮</span>
      </template>
    </el-table-column>

    <el-table-column prop="status" label="状态"  >
      <template #default="scope">
        <el-tag type="success" v-if="scope.row.status==0">开启</el-tag>
        <el-tag type="danger" v-if="scope.row.status!=0">关闭</el-tag>
      </template>
    </el-table-column>
    <el-table-column label="操作">
      <template #default="scope">
        <el-button type="primary" :icon="Edit" circle plain @click="edit(scope.row)" />
        <el-button type="danger" :icon="Delete" circle plain @click="deleteHandler(scope.row)" />
      </template>
    </el-table-column>
  </el-table>

  <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500" >
    <el-form v-model="form" label-width="auto">
      <el-form-item label="父菜单"  >
        <el-cascader clearable v-model="form.parentId" :options="menuList" :props="{label:'menuName',value:'menuId',checkStrictly:true}"></el-cascader>
      </el-form-item>
      <el-form-item label="名称"  >
        <el-input v-model="form.menuName"></el-input>
      </el-form-item>
      <el-form-item label="类型"  >
        <el-select v-model="form.type">
          <el-option :value="1" label="目录"></el-option>
          <el-option :value="2" label="菜单"></el-option>
          <el-option :value="3" label="按钮"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="地址"  >
        <el-input v-model="form.path"></el-input>
      </el-form-item>
      <el-form-item label="图标"  >
        <el-input v-model="form.icon"></el-input>
      </el-form-item>
      <el-form-item label="状态"  >
        <el-select v-model="form.status">
          <el-option :value="0" label="开启"></el-option>
          <el-option :value="1" label="关闭"></el-option>
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="dialogVisible = false">取消</el-button>
        <el-button type="primary" @click="dialogSubmit">确定</el-button>
      </div>
    </template>
  </el-dialog>
</template>
<script setup lang="ts">
import {save, list, update, del} from "~/api/menu";
  import {Delete, Edit} from "@element-plus/icons-vue";

  const menuList = ref([]);

  onMounted(async () => {
    menuList.value = (await list()).data;
  });

  const dialogVisible = ref(false);
  const dialogTitle = ref("");
  const form = ref({});
  const add = function () {
    form.value = {};
    dialogTitle.value = "新增";
    dialogVisible.value = true;
  };
const refresh = async function () {
  menuList.value = [];
  menuList.value = (await list()).data;
  console.log(menuList.value)
};
  const edit = function (t: any) {
    form.value = {};
    Object.assign(form.value, t);
    dialogTitle.value = "编辑";
    dialogVisible.value = true;
  };
  const dialogSubmit = async function () {
    let pid = form.value.parentId;
    if (pid instanceof Array && pid.length != 0) {
      form.value.parentId = pid[pid.length - 1];
    }
    let data,msg;
    if (dialogTitle.value == "新增") {
       data = await save(form.value);
      msg = "新增成功";
    }else {
      data = await update(form.value);
      msg = "更新成功";
    }
    if (data.code == "0") {
      ElMessage({message:msg, type: "success"});
      dialogVisible.value = false;
      menuList.value = (await list()).data;
    }else {
      ElMessage({message:data.msg, type: "error"});
    }


  };

const deleteHandler = async function (t: any) {
   ElMessageBox.confirm('确认删除吗？', '删除').then(async () => {
    let data = await del(t.menuId);
    if (data.code == 0) {
      ElMessage({message: "删除成功", type: "success"});
      menuList.value = (await list()).data;
    }else {
      ElMessage({message:data.msg, type: "error"});
    }
  });

};
</script>
<style scoped lang="css">

</style>
