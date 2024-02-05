# Rect

Draws a rectangle.

## Props

| Name   | Type     | Description              |
| :----- | :------- | :----------------------- |
| x      | `number` | X coordinate.            |
| y      | `number` | Y coordinate.            |
| width  | `number` | Width of the rectangle.  |
| height | `number` | Height of the rectangle. |



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
registerElement("Rect", () => require("@nativescript/canvas/Dom").Rect)
```

== Solid


== React


== Svelte


:::

## Usage

:::tabs key:flavor
== JS/TS

```vue
<template>
  <Dom>
    <Rect x="25" y="25" width="256" height="256" color="lightblue" />
  </Dom>
</template>
```

== Angular

```vue
<template>
  <Dom>
    <Rect x="25" y="25" width="256" height="256" color="lightblue" />
  </Dom>
</template>
```

== Vue

```vue
<template>
  <Dom>
    <Rect x="25" y="25" width="256" height="256" color="lightblue" />
  </Dom>
</template>
```

== Solid

```vue
<template>
  <Dom>
    <Rect x="25" y="25" width="256" height="256" color="lightblue" />
  </Dom>
</template>
```

== React

```vue
<template>
  <Dom>
    <Rect x="25" y="25" width="256" height="256" color="lightblue" />
  </Dom>
</template>
```

== Svelte

```vue
<template>
  <Dom>
    <Rect x="25" y="25" width="256" height="256" color="lightblue" />
  </Dom>
</template>
```

:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/rect.webp"/>
