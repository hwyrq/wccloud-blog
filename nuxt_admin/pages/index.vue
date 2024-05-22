

<template>
  <NuxtLayout :name="layout">
    <NuxtPage />
  </NuxtLayout>
</template>
<script setup lang="ts">
/**
 * @author wcz
 */
const layout = ref();
let route = useRoute();
if (route.path == "/") {
  useRouter().push("/home");
}
if (route.path == "/home") {
  layout.value = "admin";
}else {
  layout.value = "none";
}

</script>
<style scoped lang="css">

</style>
