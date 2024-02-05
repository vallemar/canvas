# Group


Group components can be deeply nested with one another. It can apply the following operations to its children:

- [Paint Properties](#paint-properties).
- [Transformations](#transformations).

// TODO: set Transform type link
| Name       | Type        | Description                                                                                                              |
| :--------- | :---------- | :----------------------------------------------------------------------------------------------------------------------- |
| transform? | `Transform` | The origin property is a helper to set the origin of the transformation. This property is not inherited by its children. |
| origin?    | `Point`     | Sets the origin of the transformation. This property is not inherited by its children.                                   |


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
registerElement("Group", () => require("@nativescript/canvas/Dom").Group)
```

== Solid


== React


== Svelte


:::

<br>

## Paint Properties

Its children will inherit all paint attributes applied to a group. These attributes can be properties like `color`, `paintStyle` or `strokeWidth`.

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

<br>

## Transformations

The origin property is a helper to set the origin of the transformation. This property is not inherited by its children.

:::tabs key:flavor
== JS/TS



== Angular


== Vue

```vue
<template>
  <Dom>
    <Fill color="#e8f4f8" />
    <Group color="lightblue" :transform="[{ skewX: Math.PI / 6 }]">
        <RoundedRect x="32" y="32" width="192" height="192" r="32" color="lightblue">
            <Shadow dx="12" dy="12" blur="25" color="#93b8c4" />
        </RoundedRect>
    </Group>
  </Dom>
</template>
```

== Solid


== React



== Svelte



:::
<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/transformations.webp"/>
