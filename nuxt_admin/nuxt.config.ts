// https://nuxt.com/docs/api/configuration/nuxt-config


export default defineNuxtConfig({
  devtools: {enabled: true},
  modules: [
    '@element-plus/nuxt',
  ],
  elementPlus: { /** Options */ },
  css: [
    'element-plus/dist/index.css',
  ],
/*  routeRules:{
    "/":{prerender: true }
  }*/
  runtimeConfig:{
    public: {
      a_target: "_self",
    },

  },
  vite: {
    optimizeDeps: {
      include: ['dayjs', 'dayjs/plugin/*', 'element-plus']
    }
  },
})


