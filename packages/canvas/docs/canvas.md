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

