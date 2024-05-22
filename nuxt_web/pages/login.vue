<template>
  <div class="main">
    <el-space>
      <el-card style="width: 350px">
        <p>后台</p>
        <el-form :model="form" label-width="60">
          <el-form-item label="用户名">
            <el-input v-model="form.username"></el-input>
          </el-form-item>
          <el-form-item label="密码">
            <el-input type="password" show-password v-model="form.password"></el-input>
          </el-form-item>
          <el-button type="primary" @click="submitLogin">登录</el-button>
<!--          <el-button @click="registerPage">注册</el-button>-->
        </el-form>
      </el-card>
    </el-space>
  </div>


</template>
<script setup lang="ts">
import {login} from "~/api/login";
import request from "~/utils/request";

const form = ref({username: '', password: ''});

const submitLogin = async function () {
  const data = await login(form.value);
  if (data.code == 0) {
    ElMessage({message: data.msg, type: 'success'});
    localStorage.setItem("accessToken", data.data.accessToken);
    localStorage.setItem("refreshToken", data.data.refreshToken);
    localStorage.setItem("expireTime", data.data.expireTime);
    localStorage.setItem("nickname", data.data.nickname);
    location.href = "/home";
  }else {
    ElMessage({message:data.msg, type: 'error'})
  }
};
const registerPage = function () {

};

</script>
<style scoped lang="css">
.main {
  text-align: center; /*让div内部文字居中*/
  background-color: #fff;
  border-radius: 20px;
  width: 300px;
  height: 350px;
  margin: auto;
  position: absolute;
  top: 0;
  left: -100px;
  right: 0;
  bottom: 0;
}

</style>
