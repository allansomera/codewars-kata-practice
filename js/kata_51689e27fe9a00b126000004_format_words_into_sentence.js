function formatWords(words) {
  // let match = str.replace(/(?:.+)(,)$/, " and $1")
  // let str = ""
  // if (typeof words !== "null" && Array.isArray(words))
  if (words === null || words === "") return ""
  if (Array.isArray(words)) {
    if (words.length === 0) return ""
    else if (words.length === 1 && words[0] === "") return ""
    else {
      let filter = words.filter((i) => i !== "")
      let str = words.filter((i) => i !== "").toString()

      if (filter.length === 1) return str.toString()
      else {
        str = words.filter((i) => i !== "").toString()
        let match = str.replace(/,/gm, ", ").replace(/,([^,]+)$/, " and$1")
        return match
      }
    }
  }

  // let filter = words.filter((i) => i !== "")
  // let str = words.filter((i) => i !== "").toString()
  //
  // if (filter.length === 1) return str.toString()
  // else {
  //   str = words.filter((i) => i !== "").toString()
  //   let match = str.replace(/,/gm, ", ").replace(/,([^,]+)$/, " and$1")
  //   return match
  // }

  // console.log(str)
  // console.log(match)
}

// console.log(formatWords(["one", "two", "three", "four"]))
console.log(formatWords(["one", "", "three"]))
// console.log(formatWords(null))
