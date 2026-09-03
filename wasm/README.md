JS bindings for the aemark parser, built upon Rust and WASM.

Exemple :

```javascript
import { markdown } from "@ae_utbm/aemark"

const result = markdown("This some *text* formatted in __markdown__");
console.log(result);
// <p>This some <em>text</em> formatted in <u>markdown</u></p>\n
```