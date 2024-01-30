# RoundedRect

Draws a rounded rectangle.

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/rounded_rect.webp"/>

## Props

| Name   | Type     | Description               |
| :----- | :------- | :------------------------ |
| x      | `number` | X coordinate.             |
| y      | `number` | Y coordinate.             |
| width  | `number` | Width of the rectangle.   |
| height | `number` | Height of the rectangle.  |
| r?     | `number` | Corner radius. Default 0. |


## Register element
:::tabs key:flavor
== JS/TS

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

== Angular

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

== Solid

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

== React

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

== Svelte

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

:::

## Usage

:::tabs key:flavor
== JS/TS

```vue
<template>
  <Dom>
    <RoundedRect 
        x="25" 
        y="25"
        width="256" 
        height="256" 
        r="25" 
        color="lightblue" 
    />  
  </Dom>
</template>
```

== Angular

```vue
<template>
  <Dom>
    <RoundedRect 
        x="25" 
        y="25"
        width="256" 
        height="256" 
        r="25" 
        color="lightblue" 
    />  
  </Dom>
</template>
```

== Vue

```vue
<template>
  <Dom>
    <RoundedRect 
        x="25" 
        y="25"
        width="256" 
        height="256" 
        r="25" 
        color="lightblue" 
    />  
  </Dom>
</template>
```

== Solid

```vue
<template>
  <Dom>
    <RoundedRect 
        x="25" 
        y="25"
        width="256" 
        height="256" 
        r="25" 
        color="lightblue" 
    />  
  </Dom>
</template>
```

== React

```vue
<template>
  <Dom>
    <RoundedRect 
        x="25" 
        y="25"
        width="256" 
        height="256" 
        r="25" 
        color="lightblue" 
    />  
  </Dom>
</template>
```

== Svelte

```vue
<template>
  <Dom>
    <RoundedRect 
        x="25" 
        y="25"
        width="256" 
        height="256" 
        r="25" 
        color="lightblue" 
    />
  </Dom>
</template>
```

:::
