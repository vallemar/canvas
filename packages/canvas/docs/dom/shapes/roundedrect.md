# RoundedRect

Draws a rounded rectangle.


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
registerElement("RoundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
```

== Solid


== React


== Svelte

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement("dom", () => require("@nativescript/canvas/Dom").Dom)
registerNativeViewElement("roundedRect", () => require("@nativescript/canvas/Dom").RoundedRect)
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

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/rounded_rect.webp"/>
