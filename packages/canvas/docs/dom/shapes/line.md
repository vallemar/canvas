# Line

Draws a line between two points.

## Props

| Name        | Type     | Description                                     |
| :---------- | :------- | :---------------------------------------------- |
| p1          | `Point`  | Start point.                                    |
| p2          | `Point`  | End point.                                      |
| paintStyle  | `string` | `fill` or `stroke`                              |
| strokeWidth | `number` | Thickness of the pen used to outline the shape. |


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
registerElement("Line", () => require("@nativescript/canvas/Dom").Line)
```

== Solid


== React


== Svelte

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement("dom", () => require("@nativescript/canvas/Dom").Dom)
registerNativeViewElement("line", () => require("@nativescript/canvas/Dom").Line)
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
    <Line :p1="{ x: 0, y: 0 }" :p1="{ x: 260, y: 260 }" color="lightblue" paintStyle="stroke" strokeWidth="1" />
  </Dom>
</template>
```

== Solid


== React



== Svelte



:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/line.webp"/>
