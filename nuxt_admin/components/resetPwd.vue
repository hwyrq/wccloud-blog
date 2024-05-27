

<template>
  <el-dialog v-model="dialogVisible" :title="dialogTitle" width="500" >
    <el-form :model="form" label-width="auto" :rules="rules" ref="formRef">
      <el-form-item label="旧密码"  prop="oldPass">
        <el-input type="password" show-password v-model="form.oldPwd"></el-input>
      </el-form-item>
      <el-form-item label="新密码"  prop="newPass">
        <el-input type="password" show-password v-model="form.newPwd"></el-input>
      </el-form-item>
      <el-form-item label="确认"  prop="newPass2">
        <el-input type="password" show-password v-model="form.newPwd2"></el-input>
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
import type {FormInstance} from "element-plus";
import {resetPwd} from "~/api/login";

/**
 * @author wcz
 */
const dialogVisible = ref(false);
const dialogTitle = ref("修改密码");
const form = ref({});
const rules = reactive({
  oldPwd:[{required: true, message: '', trigger: 'blur'}],
  newPwd:[{required: true, message: '', trigger: 'blur'}],
  newPwd2:[{required: true, message: '', trigger: 'blur'}],
});
const formRef = ref<FormInstance>();
const dialogSubmit =async function () {
  if (form.value.Pwd != form.value.Pwd2) {
    ElMessage({message:"两次输入的密码不一致",type:"error"});
    return;
  }else{
    await formRef.value.validate(async (valid, fields) => {
      if (valid) {
        let data =await resetPwd({pwd: form.value.oldPwd, newPwd: form.value.newPwd});
        console.log(data)
        if (data.code == 0) {
          ElMessage({message: "修改成功", type: "success"});
          form.value = {};
          dialogVisible.value = false;
        } else {
          ElMessage({message: data.msg, type: "error"});
        }
      }
    });
  }

};
defineExpose({
  setDialogVisible(arg:any){
    form.value = {};
    dialogVisible.value = arg;
  }

});
</script>
<style scoped lang="css">

</style>