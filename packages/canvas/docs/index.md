---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "NativeScript Canvas"
  text: "Native Canvas for NativeScript"
  _tagline: My great project tagline
  actions:
    - theme: brand
      text: Get Started
      link: /installation
    - theme: alt
      text: Examples
      link: /examples

features:
  - title: Totally Native
    details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
  - title: Familiar
    details: NativeScript Canvas implements the same API as html canvas
  - title: WEBGL
    details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
  - title: 2D & 3D
    details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
---


<script setup>
import { VPTeamMembers } from 'vitepress/theme'
import Preview from './.vitepress/theme/components/Preview.vue'

const members = [
  {
    avatar: 'https://avatars.githubusercontent.com/u/6695919?v=4',
    name: 'triniwiz',
    title: 'Creator',
    links: [
      { icon: 'github', link: 'https://github.com/triniwiz' },
      { icon: 'twitter', link: 'https://twitter.com/triniwiz' }
    ]
  },

]
</script>

<Preview />

<div class="flex justify-center mt-12">
<h3 class="text-2xl">Team</h3>
</div>



<VPTeamMembers size="small" :members="members" />