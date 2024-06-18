
<template >
  <div style="margin: 5px">
    <el-button plain type="success" @click="saveHandler" >保存</el-button>
  </div>
  <el-form :model="form" ref="formRef" :rules="rules" :inline="true" status-icon label-width="auto">
    <el-row style="" :gutter="10" >
      <el-col :span="20" style="">
        <el-form-item label="标题" prop="title" style="width:100%">
          <el-input v-model="form.title" type="text"/>
        </el-form-item>
        <el-form-item label="简介" prop="summary" style="width:100%">
          <el-input v-model="form.summary"  type="text"/>
        </el-form-item>
        <el-form-item label="分类"  prop="typeName" style="width: 200px">
          <el-select v-model="form.typeName"  allow-create filterable :reserve-keyword="false">
            <el-option v-for= "item in typeNameList " :label="item.typeName" :value="item.typeName"></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="标签" prop="labelName" style="width: 500px">
          <el-select v-model="form.labelName" multiple allow-create filterable :reserve-keyword="false">
            <el-option v-for= "item in labelNameList " :label="item.labelName" :value="item.labelName"></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="推荐" prop="level" style="width: 200px">
          <el-select v-model="form.level"  >
            <el-option label="一级" :value="1"></el-option>
            <el-option label="二级" :value="2"></el-option>
            <el-option label="三级" :value="3"></el-option>
            <el-option label="否" :value="0"></el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="是否评论" prop="enableComment">
          <el-switch v-model="form.enableComment"></el-switch>
        </el-form-item>
        <el-form-item label="状态" prop="status" style="width: 200px">
          <el-select v-model="form.status"  >
            <el-option label="发布" :value="0"></el-option>
            <el-option label="下架" :value="1"></el-option>
          </el-select>
        </el-form-item>
      </el-col>
      <el-col :span="3" style="">
        <upload-img ref="uploadRef"/>
      </el-col>
    </el-row>

    <el-row>

    </el-row>



  </el-form>
<div id="vditor" :style="vdStyle"></div>
</template>
<script setup lang="ts">
/**
 * @author wcz
 */
import Vditor from "vditor";
import 'vditor/dist/index.css'
import {listLabelName, listTypeName, one, save} from "~/api/blog";
import type {FormInstance, FormRules} from "element-plus";

const form = ref({level:0,enableComment: true, status: 0,html:'',md:''});
const formRef = ref<FormInstance>();
const uploadRef = ref(null);
const rules = reactive({
  title:[{required: true, message: '', trigger: 'blur'}],
  summary:[{required: true, message: '', trigger: 'blur'}],
  typeName:[{required: true, message: '', trigger: 'blur'}],
  labelName:[{required: true, message: '', trigger: 'blur'}],
  level:[{required: true, message: '', trigger: 'blur'}],
  enableComment:[{required: true, message: '', trigger: 'blur'}],
  status:[{required: true, message: '', trigger: 'blur'}],
});
const labelNameList = ref([]);
const typeNameList = ref([]);
let vd: Vditor | null = null;
let vdStyle = ref({height: "0px"});
onMounted(async () => {

  localStorage.setItem("vditorvditor","");
  let item = localStorage.getItem("vditorvditor-mode");
  if (item == null || item == "") {
    item = "wysiwyg";
  }
  vd = new Vditor('vditor', {
    placeholder: '正文......',
    mode: item,
    counter: {enable: true, type: 'markdown'},
    upload:{
      url:request.defaults.baseURL+ "/wccloud-web-rust/file/upload",
      headers:{
        ["Token"]: localStorage.getItem("accessToken"),
      },

      success(editor, msg) {
        console.log(editor);
        let res = JSON.parse(msg);
        if (res.code != 0) {
          ElMessage({message: "上传失败", type: "error"});
        } else {
            for (const url of res.data) {
              vd?.setValue(vd?.getValue() + "![]("+url+")");
            }
        }
      },
    }
  });
  vdStyle.value = {height: window.innerHeight - 308 + "px"}
  window.addEventListener("resize", () => {
    vdStyle.value = {height: window.innerHeight - 308 + "px"}
  });
  listTypeName().then((res)=>{
    typeNameList.value = res.data;
  })
  listLabelName().then((res)=>{
    labelNameList.value = res.data;
  })

  let query = useRouter().currentRoute.value.query;
  if (query.id) {
    one({blogId: query.id}).then((res)=>{
      form.value = res.data;
      vd?.setValue(form.value.md, true);
      if (form.value.imgUrl != null && form.value.imgUrl != "" && uploadRef != null) {
        uploadRef.value.url = form.value.imgUrl;
        uploadRef.value.fileList.push({
          name: '1.jpeg',
          url: form.value.imgUrl,
        });
      }
    })
  }

});

const saveHandler = async function () {
  //首先保存一下编辑模式，以便下次切换
 localStorage.setItem("vditorvditor-mode",vd.getCurrentMode());
  await formRef.value.validate(async (valid, fields) => {
    if (valid) {
      form.value.html = <string>vd?.getHTML();
      form.value.md = <string>vd?.getValue();
      form.value.imgUrl = uploadRef.value.url;
      let data = await save(form.value);
      if (data.code == 0) {
        ElMessage({message: "保存成功", type: "success"});
      } else {
        ElMessage({message: data.msg, type: "error"});
      }
    }
  });

};
</script>

<style scoped lang="css">

</style>
