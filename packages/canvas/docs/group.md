# Group


Group components can be deeply nested with one another. It can apply the following operations to its children:

- [Transformations](#transformations).

// TODO
| Name       | Type          | Description                                                                                                                                                                                                                  |
| :--------- | :------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| transform? | `Transform2d` | [Same API that's in React Native](https://reactnative.dev/docs/transforms). The default origin of the transformation is, however, different. It is the center object in NativeScript Canvas and the top-left corner in Skia. |
| origin?    | `Point`       | Sets the origin of the transformation. This property is not inherited by its children.                                                                                                                                       |


## Register element
:::tabs key:flavor
== JS/TS

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

== Angular

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

== Solid

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

== React

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

== Svelte

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

:::

## Usage


:::tabs key:flavor
== JS/TS



== Angular


== Vue

```vue
<script lang="ts" setup>
import { ref } from 'nativescript-vue';

const r = ref(60);
</script>

<template>
  <Dom>
    <Circle :cx="r" :cy="r" :r="r" color="#65adf1" />
    <Group color="lightblue" paintStyle="stroke" strokeWidth="10">
      <Circle :cx="r" :cy="r" :r="5" />
      <Circle :cx="r" :cy="r" :r="r / 2" />
      <Circle :cx="r" :cy="r" :r="r / 3" color="white" />
    </Group>

    <Group :origin="{ x: 80, y: 80 }" color="#bd34fe" paintStyle="stroke" strokeWidth="4">
      <Circle :cx="r" :cy="r" :r="5" />
      <Circle :cx="r" :cy="r" :r="r / 3" />
    </Group>
  </Dom>
</template>
```

== Solid


== React



== Svelte



:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/group.webp"/>


## Transformations