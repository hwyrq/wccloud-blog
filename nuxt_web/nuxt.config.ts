// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  devtools: {enabled: true},
  modules: [
    '@element-plus/nuxt',
    "@nuxtjs/sitemap",
  ],
  css: [
    'element-plus/dist/index.css',
  ],
/*  routeRules:{
    "/":{prerender: true }
  }*/
/*  app: {
    baseURL:"/web",
  },*/
  runtimeConfig:{
    public: {
      a_target: "_self",
      baseUrl: process.env.BASE_URL,
      baseUrlIn: process.env.BASE_URL_IN
    },
  },

  vite: {
    optimizeDeps: {
      include: ['dayjs', 'dayjs/plugin/*', 'element-plus']
    }
  },
})


