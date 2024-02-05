# Path

TODO


## Props

| Name | Type     | Description                                                                                                                                 |
| :--- | :------- | :------------------------------------------------------------------------------------------------------------------------------------------ |
| path | `string` | Path to draw. Can be a string using the [SVG Path notation](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#line_commands). |



## Register element
:::tabs key:flavor
== JS/TS

```xml
<Page xmlns:ui="@nativescript/canvas/dom/index" xmlns="http://schemas.nativescript.org/tns.xsd">
     <!-- ... -->
</Page>
```

== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Path", () => require("@nativescript/canvas/Dom").Path)
```

== Solid


== React


== Svelte

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement("dom", () => require("@nativescript/canvas/Dom").Dom)
registerNativeViewElement("path", () => require("@nativescript/canvas/Dom").Path)
```

:::

## Usage

:::tabs key:flavor
== JS/TS



== Angular


== Vue

```vue
<template>
  <Dom>
    <Path 
        path="M 128 0 L 168 80 L 256 93 L 192 155 L 207 244 L 128 202 L 49 244 L 64 155 L 0 93 L 88 80 L 128 0 Z" 
        color="lightblue" 
    />
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/path.webp"/>

