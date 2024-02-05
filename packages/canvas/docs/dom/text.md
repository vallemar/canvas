# Text

The text component can be used to draw a simple text.
Please note that the y origin of the Text is the bottom of the text, not the top.


## Props

| Name | Type     | Description                                   |
| :--- | :------- | :-------------------------------------------- |
| text | `string` | Text to draw                                  |
| font | `string` | Font to use                                   |
| x    | `number` | Left position of the text (default is 0)      |
| y    | `number` | Bottom position the text (default is 0, the ) |



## Register element
:::tabs key:flavor
== JS/TS


== Angular


== Vue

```ts
import { registerElement } from 'nativescript-vue';

registerElement("Dom", () => require("@nativescript/canvas/Dom").Dom)
registerElement("Text", () => require("@nativescript/canvas/Dom").Text)
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
  <Dom>
    <Text x="25" y="60" text="Hello World" font="bold 32px serif" />
  </Dom>
</template>
```

== Solid


== React


== Svelte


:::

<img height="300px" width="300px" style="margin-bottom: 12px;" src="/img/text.webp"/>
