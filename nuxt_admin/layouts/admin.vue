<template>
  <div class="my-home">
    <el-row >
      <el-col :span="2" :style="menuStyle" class="menuStyle">
        <el-menu :collapse="isCollapse" @select="select" :default-active="defaultActive"	 >
          <menu-tree :listMenu="listMenu" ></menu-tree>
        </el-menu>
      </el-col>

      <el-col :span="22" style="padding-left: 5px" >
          <el-row  style="width: 100%;height:40px;border:1px solid #dcdfe6;margin-bottom:5px" >
            <el-col :span="23" >
              <el-breadcrumb :separator-icon="ArrowRight"  style="margin-top:12px;margin-left:15px">
                <el-breadcrumb-item  v-for="item in breadcrumb" :to="{ path: item.path }">{{ item.menuName }}</el-breadcrumb-item>
              </el-breadcrumb>
            </el-col>
            <el-col :span="1">
              <el-dropdown @command="handleCommand">
                <el-row>
                  <el-col :span="12">

                    <el-avatar  :size="30" src="https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png"/>
                  </el-col>
                  <el-col :span="12">
                    <div style="margin-top:10px">{{nickname}}</div>
                  </el-col>
                </el-row>
                <template #dropdown>
                  <el-dropdown-menu>
                    <el-dropdown-item >个人中心</el-dropdown-item>
                    <el-dropdown-item command="resetPassword">修改密码</el-dropdown-item>
                    <el-dropdown-item command="logout">退出系统</el-dropdown-item>
                  </el-dropdown-menu>
                </template>
              </el-dropdown>
            </el-col>

          </el-row>
          <el-row  style="border:1px solid #dcdfe6" >
            <div style="width: 100%;" >
              <el-tabs style="width: 100%;" type="card" v-model="tabsValue" closable @tabClick="tabHandler" @tabRemove="tabRemove">
                <el-tab-pane  style="height:0" v-for="item in tabs" :key="item.key" :label="item.label" :name="item.name"></el-tab-pane>
              </el-tabs>
            </div>

          </el-row>
        <slot/>
      </el-col>
    </el-row>
  </div>
<reset-pwd ref="pwdRef"/>
</template>

<script setup lang="ts">
/**
 * @author wcz
 */
import {listStatus0} from "~/api/menu";
import router from "#app/plugins/router";
import {useAsyncData, useRoute} from "#app";
import nuxtLink from "#app/components/nuxt-link";
import {ArrowRight} from "@element-plus/icons-vue";
import {logout} from "~/api";
import MenuTree from "~/components/menuTree.vue";
import type {TabPaneName, TabsPaneContext} from "element-plus";

let instance = getCurrentInstance();
const listMenu = ref([]);
const data = await listStatus0();
listMenu.value = data.data;
const nickname = ref();
const menus: any[] = [];
const menuMap = {};
const menuStyle = ref({height:"500px"});
const isCollapse = ref(false);
const findby = function (item) {
  menus.push(item);
  if (item.children != null) {
    for (const sub of item.children) {
      findby(sub);
    }
  }
};
for (const item of listMenu.value) {
  findby(item);
}

const tabs = ref([]);
const tabsValue = ref("");
onMounted(async () => {
  window.addEventListener("resize", () => {
    menuStyle.value.height = window.innerHeight-30 +"px";
  });
  menuStyle.value.height = window.innerHeight-30 +"px";
  nickname.value = localStorage.getItem("nickname");
});



const breadcrumb = ref([]);
const pwdRef = ref(null);
const handleCommand = async function (command: string) {
  if (command == "logout") {
    let data = await logout();
    ElMessage({message: data.msg, type: 'success'});
    location.href = "/login";
  }else if (command == "resetPassword") {
    pwdRef.value.setDialogVisible(true);
  }

};

const findMenuTo = function (index) {
  //找到当前菜单并跳转
  for (const menu of menus) {
    if (menu.menuId == index) {
      useRouter().push(menu.path);
      if (menuMap[menu.menuId] == null) {
        tabs.value.push({key:menu.menuId,label:menu.menuName,name:menu.menuId});
        menuMap[menu.menuId] = 1;
      }
      tabsValue.value = menu.menuId;
      break;
    }
  }
};

const select = function (index, indexPath:[], item, routeResult) {
  findMenuTo(index);

  //面包屑导航展示数据
   breadcrumb.value = [];
  for (const menu of menus) {
    if (indexPath.indexOf(menu.menuId) >= 0) {
      breadcrumb.value.push({menuName: menu.menuName, path: menu.path});
    }
  }

};
const tabHandler = function (pane: TabsPaneContext, ev: Event) {
  const index = pane.props.name;
  //路由
  //找到当前菜单并跳转
  findMenuTo(index);

  defaultActive.value = index

};
const tabRemove = function (name: TabPaneName) {
  if (tabs.value.length == 1) {
    return;
  }
  console.log(name);
  tabs.value = tabs.value.filter((value, index, array)=>{
    return value.name != name
  })
  menuMap[name] = null;

  findMenuTo(tabs.value[tabs.value.length-1].name);

};

const defaultActive = ref(0);

//将菜单的第一个作为首页，不写死，
if (useRoute().path == "/") {
  if (listMenu.value.length != 0) {
    useRouter().push(listMenu.value[0].path);
    select(listMenu.value[0].menuId, [listMenu.value[0].menuId], null, null);
    defaultActive.value = listMenu.value[0].menuId
  }
}else {
  //处理浏览器刷新的情况
  for (const menu of menus) {
    if (useRoute().path == menu.path) {
      select(menu.menuId, [menu.menuId], [menu.menuId], null);
      defaultActive.value = menu.menuId;
    }
  }
}
</script>

<style  lang="css">
.my-home .el-tabs__nav{
  border-radius: 0 !important;
  border-top: 0 !important;
  border-left: 0 !important;
}
.my-home .el-tabs__header{
  margin: 0 !important;
  border: 0 !important;
}
.my-home .el-tabs__item{
}
.my-home .el-tabs__nav-prev{
  border-right: 1px solid #dcdfe6 !important;
  display:block !important;
}
.my-home .el-tabs__nav-next{
  border-left: 1px solid  #dcdfe6 !important;
}
.my-home .el-col{
  overflow-y: scroll;
}
.my-home .el-col::-webkit-scrollbar{
  display: none !important;
}
.my-home .el-breadcrumb__item{
  cursor:pointer !important;
}
.my-home .menuStyle{
  border: 1px solid #dcdfe6;
}
.my-home .el-menu{
  border-right: 0;
}
</style>
