# Gradients

## Linear Gradient

### Props

| Name   | Type       | Description                                     |
| :----- | :--------- | :---------------------------------------------- |
| start  | `Point`    | Start position of the gradient.                 |
| end    | `Point`    | End position of the gradient.                   |
| colors | `string[]` | Colors to be distributed between start and end. |



### Register element

:::tabs key:flavor
== JS/TS


== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("LinearGradient", () => require("@nativescript/canvas/Dom").LinearGradient)
```

== Solid


== React


== Svelte


:::

### Usage

:::tabs key:flavor
== JS/TS



== Angular


== Vue

```vue
<template>
  <Dom>
    <Rect x="0" y="0" width="256" height="256">
      <LinearGradient :start="{ x: 0, y: 0 }" :end="{ x: 256, y: 256 }" :colors="['#bd34fe', '#65adf1']" />
    </Rect>
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/linear_gradient.webp"/>


## Two Point Conical Gradient


### Props

| Name   | Type       | Description                                     |
| :----- | :--------- | :---------------------------------------------- |
| start  | `Point`    | Center of the start circle.                     |
| startR | `number`   | Radius of the start circle.                     |
| end    | `Point`    | Center of the end circle.                       |
| endR   | `number`   | Radius of the end circle.                       |
| colors | `string[]` | Colors to be distributed between start and end. |



### Register element
:::tabs key:flavor
== JS/TS


== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("TwoPointConicalGradient", () => require("@nativescript/canvas/Dom").TwoPointConicalGradient)
```

== Solid


== React


== Svelte


:::

### Usage

:::tabs key:flavor
== JS/TS



== Angular


== Vue

```vue
<template>
  <Dom>
    <Rect x="0" y="0" width="256" height="256">
      <TwoPointConicalGradient 
        :start="{ x: 0, y: 0 }" 
        startR="128" 
        :end="{ x: 256, y: 256 }" 
        endR="16" 
        :colors="['#bd34fe', '#65adf1']" 
      />
    </Rect>
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/two_point_conical_gradient.webp"/>
