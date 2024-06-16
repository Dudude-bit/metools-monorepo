// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@nuxt/ui", "@pinia/nuxt", "@nuxtjs/device"],
  runtimeConfig: {
    public: {
      baseApiURL: process.env.BASE_URL || 'http://localhost:8000/',
    },
  },
})
// TODO change base url depending on if dev