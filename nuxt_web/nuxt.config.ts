// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  devtools: {enabled: true},
  modules: [
    '@element-plus/nuxt',
  ],
  css: [
    'element-plus/dist/index.css',
  ],
/*  routeRules:{
    "/":{prerender: true }
  }*/
  runtimeConfig:{
    public: {
      a_target: "_self",
      basePATH: "http://wccloud.top"
    },

  },
  vite: {
    optimizeDeps: {
      include: ['dayjs', 'dayjs/plugin/*', 'element-plus']
    }
  },
})

