export default defineNuxtRouteMiddleware((to) => {
    // 在你要导航到的路由上设置布局
    if (to.path == "/login") {
        setPageLayout('none')
    }else {
        setPageLayout('admin')
    }
})
