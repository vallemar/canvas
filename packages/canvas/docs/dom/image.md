# Image

Images can be drawn by specifying the output rectangle and how the image should fit into that rectangle.

## Props
// TODO ImageSource???
| Name   | Type             | Description                                 |
| :----- | :--------------- | :------------------------------------------ |
| image  | `ImageSource???` | An instance of the image.                   |
| x      | `number`         | The left position of the destination image. |
| y      | `number`         | The top position of the destination image.  |
| width  | `number`         | The width of the destination image.         |
| height | `number`         | The height of the destination image.        |



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
registerElement("ImageCanvas", () => require("@nativescript/canvas/Dom").Image)
```

== Solid


== React


== Svelte


:::

## Usage

:::tabs key:flavor
== JS/TS

```xml
<Page xmlns:ui="@nativescript/canvas/dom/index" xmlns="http://schemas.nativescript.org/tns.xsd">
     <!-- ... -->
</Page>
```

== Angular

```vue
<script lang="ts" setup>
import { ImageSource } from '@nativescript/core';

const image = ImageSource.fromFileSync("~/assets/NativeScript_Logo.png");
</script>

<template>
  <Dom>
    <ImageCanvas :image="image" fit="contain" x="150" y="0" width="256" height="256" />
  </Dom>
</template>
```

== Vue

```vue
<script lang="ts" setup>
import { ImageSource } from '@nativescript/core';

const image = ImageSource.fromFileSync("~/assets/NativeScript_Logo.png");
</script>

<template>
  <Dom>
    <ImageCanvas :image="image" fit="contain" x="150" y="0" width="256" height="256" />
  </Dom>
</template>
```

== Solid

```vue
<script lang="ts" setup>
import { ImageSource } from '@nativescript/core';

const image = ImageSource.fromFileSync("~/assets/NativeScript_Logo.png");
</script>

<template>
  <Dom>
    <ImageCanvas :image="image" fit="contain" x="150" y="0" width="256" height="256" />
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

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/image.webp"/>
