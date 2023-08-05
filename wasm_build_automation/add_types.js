const fs = require('fs')
const path = "pkg/package.json"
const pkg = JSON.parse(fs.readFileSync(path))
pkg.files = pkg.files.concat("types") 
fs.writeFileSync("pkg/package.json", JSON.stringify(pkg, null, 2));