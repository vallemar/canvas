# Fill

Fill canvas or group.

## Props

| Name  | Type     | Description    |
| :---- | :------- | :------------- |
| color | `string` | Color to paint |


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
registerElement("Fill", () => require("@nativescript/canvas/Dom").Image)
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
<template>
  <Dom height="200">
    <Fill color="#65adf1" />
  </Dom>
</template>
```

== Solid



== React



== Svelte



:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/fill.webp"/>
