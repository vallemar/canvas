import { defineConfig } from 'vitepress'
import { tabsMarkdownPlugin } from 'vitepress-plugin-tabs'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  head: [
    [
      'script',
      {
        async: 'false',
        src: 'https://unpkg.com/@vueuse/core',
      },
    ],
  ],
  title: "NativeScript Canvas",
  description: "Native Canvas for NativeScript",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Get Started', link: '/installation' },
      { text: 'Playground', link: 'https://stackblitz.com/edit/nativescrip-use-vue?file=src%2Fapp.ts' },
    ],

    sidebar: [
      {
        text: 'Guide',
        items: [{ text: 'Intallation', link: '/installation' }],
      },
      {
        text: 'Canvas',
        link: "/canvas",
      },
      {
        text: 'Polyfills',
        link: "/polyfills",
      },
      {
        text: 'Group',
        link: "/group",
      },
      {
        collapsed: true,
        text: 'Shapes',
        items: [
          { text: 'Circle', link: '/shapes/circle' },
          { text: 'Line', link: '/shapes/line' },
          { text: 'Path', link: '/shapes/path' },
          { text: 'Points', link: '/shapes/points' },
          { text: 'Rect', link: '/shapes/rect' },
          { text: 'RoundedRect', link: '/shapes/roundedrect' },
        ],
      },
      {
        text: 'Fill',
        link: "/fill",
      },
      {
        text: 'Image',
        link: "/image",
      },
      {
        text: 'Paint',
        link: "/paint",
      },
      {
        text: 'Shadow',
        link: "/shadow",
      },
      {
        text: 'Text',
        link: "/text",
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vallemar/canvas' },
      { icon: 'discord', link: 'https://discord.com/invite/RgmpGky9GR' },
    ],
    search: {
      provider: 'local',
    },
  },
  markdown: {
    config(md) {
      md.use(tabsMarkdownPlugin)
    }
  }
})
