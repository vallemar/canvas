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
  cleanUrls: true,
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
        text: 'Dom',
        link: "/dom/index",
        collapsed: false,
        items: [

          {
            text: 'Group',
            link: "/dom/group",
          },
          {
            text: 'Shapes',
            collapsed: false,
            items: [
              { text: 'Circle', link: '/dom/shapes/circle' },
              { text: 'Line', link: '/dom/shapes/line' },
              { text: 'Gradients', link: '/dom/shapes/gradients' },
              { text: 'Path', link: '/dom/shapes/path' },
              { text: 'Points', link: '/dom/shapes/points' },
              { text: 'Rect', link: '/dom/shapes/rect' },
              { text: 'RoundedRect', link: '/dom/shapes/roundedrect' },
            ],
          },
          {
            text: 'Fill',
            link: "/dom/fill",
          },
          {
            text: 'Image',
            link: "/dom/image",
          },
          {
            text: 'Shadow',
            link: "/dom/shadow",
          },
          {
            text: 'Text',
            link: "/dom/text",
          },
        ]
      },

      {
        collapsed: false,
        text: 'Pugins',
        items: [
          //  { text: 'ECharts', link: '/charts/ECharts' },
          { text: 'Three.js', link: '/plugins/threejs' },
          { text: 'PixiJS', link: '/plugins/pixijs' },
          { text: 'Phaser', link: '/plugins/phaser' },
        ],
      },
      {
        collapsed: true,
        text: 'Charts',
        items: [
          { text: 'ECharts', link: '/charts/ECharts' },
          { text: 'Chart.js', link: '/charts/chartjs' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/vallemar/canvas' },
      { icon: 'discord', link: 'https://discord.com/invite/RgmpGky9GR' },
    ],
    search: {
      provider: 'local',
    },
    editLink: {
      pattern: 'https://github.com/NativeScript/canvas/edit/docs/:path',
      text: 'Edit this page on GitHub'
    },
  },
  lastUpdated: true,
  markdown: {
    config(md) {
      md.use(tabsMarkdownPlugin)
    }
  }
})
