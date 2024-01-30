# Circle

Draws a circle.

<img height="300px" width="300px" src="/img/circle.webp"/>

## Props

| Name | Type     | Description  |
| :--- | :------- | :----------- |
| cx   | `number` | Start point. |
| cy   | `number` | End point.   |
| r    | `number` | Radius.      |


## Register element
:::tabs key:flavor
== JS/TS

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom/index").Dom)
registerElement("Circle", () => require("@nativescript/canvas/Dom/index").Circle)
```

== Angular

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom/index").Dom)
registerElement("Circle", () => require("@nativescript/canvas/Dom/index").Circle)
```

== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom/index").Dom)
registerElement("Circle", () => require("@nativescript/canvas/Dom/index").Circle)
```

== Solid

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom/index").Dom)
registerElement("Circle", () => require("@nativescript/canvas/Dom/index").Circle)
```

== React

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom/index").Dom)
registerElement("Circle", () => require("@nativescript/canvas/Dom/index").Circle)
```

== Svelte

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom/index").Dom)
registerElement("Circle", () => require("@nativescript/canvas/Dom/index").Circle)
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

