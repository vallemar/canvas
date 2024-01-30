// https://vitepress.dev/guide/custom-theme
import { h } from 'vue'
import type { Theme } from 'vitepress'
import DefaultTheme from 'vitepress/theme'
import './style.css'
import { enhanceAppWithTabs } from 'vitepress-plugin-tabs/client'
import HeroCanvas from "./components/HeroCanvas.vue"
export default {
  extends: DefaultTheme,
  Layout: HeroCanvas,

  enhanceApp({ app, router, siteData }) {
    enhanceAppWithTabs(app)
  }
} satisfies Theme
