# Shadow

TODO

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/shadow.webp"/>

## Props

| Name  | Type     | Description                    |
| :---- | :------- | :----------------------------- |
| dx    | `number` | The X offset of the shadow.    |
| dy    | `number` | The Y offset of the shadow.    |
| blur  | `number` | The blur radius for the shadow |
| color | `Color`  | The color of the drop shadow   |



## Register element
:::tabs key:flavor
== JS/TS


== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Shadow", () => require("@nativescript/canvas/Dom").Shadow)
```

== Solid


== React


== Svelte


:::

## Usage

:::tabs key:flavor
== JS/TS



== Angular


== Vue

```vue
<template>
  <Dom>
    <RoundedRect x="32" y="32" width="192" height="192" r="32" color="lightblue">
        <Shadow dx="12" dy="12" blur="25" color="#93b8c4" />
        <Shadow dx="-12" dy="-12" blur="25" color="#c7f8ff" />
    </RoundedRect>
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::


