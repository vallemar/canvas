# Circle

Draws a circle.

## Props

| Name | Type     | Description  |
| :--- | :------- | :----------- |
| cx   | `number` | Start point. |
| cy   | `number` | End point.   |
| r    | `number` | Radius.      |


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
registerElement("Circle", () => require("@nativescript/canvas/Dom").Circle)
```

== Solid

== React


== Svelte

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement("dom", () => require("@nativescript/canvas/Dom").Dom)
registerNativeViewElement("circle", () => require("@nativescript/canvas/Dom").Circle)
```

:::


## Usage

:::tabs key:flavor
== JS/TS

```vue
<template>
  <Dom>
    <Circle cx="128" cy="128" r="128" color="lightblue"/>
  </Dom>
</template>
```

== Angular

```vue
<template>
  <Dom>
    <Circle cx="128" cy="128" r="128" color="lightblue"/>
  </Dom>
</template>
```

== Vue

```vue
<template>
  <Dom>
    <Circle cx="128" cy="128" r="128" color="lightblue"/>
  </Dom>
</template>
```

== Solid

```vue
<template>
  <Dom>
    <Circle cx="128" cy="128" r="128" color="lightblue"/>
  </Dom>
</template>
```

== React

```vue
<template>
  <Dom>
    <Circle cx="128" cy="128" r="128" color="lightblue"/>
  </Dom>
</template>
```

== Svelte

```vue
<template>
  <Dom>
    <Circle cx="128" cy="128" r="128" color="lightblue"/>
  </Dom>
</template>
```

:::

<img height="300px" width="300px" src="/img/circle.webp"/>

