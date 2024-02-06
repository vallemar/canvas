# Canvas






## Events

| Name  | Params             | Description     |
| :---- | :----------------- | :-------------- |
| ready | `{object: Canvas}` | On ready canvas |

## Register element

:::tabs key:flavor
== JS/TS

```xml
<Page xmlns:canvas="@nativescript/canvas" xmlns="http://schemas.nativescript.org/tns.xsd">
     <!-- ... -->
</Page>
```

== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Canvas", () => require("@nativescript/canvas").Canvas)
```

== Solid


== React


== Svelte

```ts
import { registerNativeViewElement } from 'svelte-native/dom'

registerNativeViewElement('canvas', () => require('@nativescript/canvas').Canvas)
```

:::


## Usage

:::tabs key:flavor
== JS/TS



== Angular



== Vue

```vue
<script lang="ts" setup>
import { EventData } from '@nativescript/core';
import type { Canvas } from '@nativescript/canvas';

function onCanvasReady(event: EventData) {
  const canvas = event.object as Canvas;
  // ...
}
</script>

<template>
  <Canvas @ready="onCanvasReady"></Canvas>
</template>
```

== Solid



== React



== Svelte



:::

