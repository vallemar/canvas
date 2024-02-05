# Fill

Fill canvas or group.

## Props

| Name  | Type     | Description    |
| :---- | :------- | :------------- |
| color | `string` | Color to paint |


## Register element
:::tabs key:flavor
== JS/TS

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Fill", () => require("@nativescript/canvas/Dom").Fill)
```

== Angular

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Fill", () => require("@nativescript/canvas/Dom").Fill)
```

== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Fill", () => require("@nativescript/canvas/Dom").Image)
```

== Solid

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Fill", () => require("@nativescript/canvas/Dom").Image)
```

== React

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Fill", () => require("@nativescript/canvas/Dom").Image)
```

== Svelte

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Fill", () => require("@nativescript/canvas/Dom").Image)
```

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
