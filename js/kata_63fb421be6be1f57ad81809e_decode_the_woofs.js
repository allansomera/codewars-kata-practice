function woofDecoder(str) {
  // Woof-woof!
  let woof = "woof"
  // let s_pattern = woof.split("").join(".*")
  // let pattern = new RegExp(s_pattern, "gi")
  // let pattern = /w.*o.*.o.*f/g
  let pattern = /woof/gi
  let split_arr = []

  if (str.includes("!")) {
    split_arr = [...str.split("!")].filter((i) => i)
    split_arr.forEach((i) => {})
  }

  console.log((str.match(pattern) || []).length)
  console.log(str.match(pattern))
  console.log(split_arr)
}

woofDecoder("aWoof-woof-woof-woof-woof-woof-woof!woof woof woof!")
// woofDecoder("awoof")
// woofDecoder("awereroereroerefer")
