import * as Pacha from "./pkg/pachadb_core_wasm.js"

let db = new Pacha.Client()

let luke = Pacha.Value.uri(new Pacha.Uri("luke"))
let anakin = Pacha.Value.uri(new Pacha.Uri("anakin"))
let ahsoka = Pacha.Value.uri(new Pacha.Uri("ahsoka"))

await db.state([
  { entity: "obiwan", field: "master-of", value: anakin, source: "test" },
  { entity: "anakin", field: "master-of", value: ahsoka, source: "test" },
])

await db.state([
  { entity: "obiwan", field: "master-of", value: luke, source: "test" },
])

let result = await db.query("(?who master-of ?pupil)")

console.log(result)

export {Pacha, db}
