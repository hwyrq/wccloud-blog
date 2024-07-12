

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



</script>
<style scoped lang="css">

</style>
