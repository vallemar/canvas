# Points

Draws points and optionally draws the connection between them.

<img height="300px" width="300px" src="/img/points.webp"/>

## Props

| Name   | Type        | Description                                                                                                                                                |
| :----- | :---------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------- |
| points | `Point[]`   | Points to draw.                                                                                                                                            |
| mode   | `PointMode` | How should the points be connected. Can be `points` (no connection), `lines` (connect pairs of points), or `polygon` (connect lines). Default is `points`. |


## Register element
:::tabs key:flavor
== JS/TS


== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Points", () => require("@nativescript/canvas/Dom").Points)
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
<script lang="ts" setup>
const points = [
  { x: 128, y: 0 },
  { x: 168, y: 80 },
  { x: 256, y: 93 },
  { x: 192, y: 155 },
  { x: 207, y: 244 },
  { x: 128, y: 202 },
  { x: 49, y: 244 },
  { x: 64, y: 155 },
  { x: 0, y: 93 },
  { x: 88, y: 80 },
  { x: 128, y: 0 },
];
</script>

<template>
  <Dom>
    <Points 
      :points="points" 
      mode="polygon" 
      color="lightblue" 
      paintStyle="stroke" 
      strokeWidth="4" 
    />
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::

