# Oval

Draws an oval based on its bounding rectangle.

## Props

| Name   | Type     | Description                             |
| :----- | :------- | :-------------------------------------- |
| x      | `number` | X coordinate of the bounding rectangle. |
| y      | `number` | Y coordinate of the bounding rectangle. |
| width  | `number` | Width of the bounding rectangle.        |
| height | `number` | Height of the bounding rectangle.       |


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
registerElement("Oval", () => require("@nativescript/canvas/Dom").Oval)
```

== Solid

== React


== Svelte

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement("dom", () => require("@nativescript/canvas/Dom").Dom)
registerNativeViewElement("oval", () => require("@nativescript/canvas/Dom").Oval)
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
    <Oval x="64" y="0" width="128" height="256" color="#65adf1" />
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::

<img height="300px" width="300px" src="/img/oval.webp"/>

