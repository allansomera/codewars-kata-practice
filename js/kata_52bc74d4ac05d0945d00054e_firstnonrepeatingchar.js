function firstNonRepeatingLetter(s) {
  if (s === "") return
  let k = {}
  s.split("").forEach((i) => {
    // if (i == i.toLowerCase()) k[i.toLowerCase()] = ++k[i.toLowerCase()] || 1
    // else if (i == i.toUpperCase())
    //   k[i.toLowerCase()] = ++k[i.toLowerCase()] || 1
    k[i.toLowerCase()] = ++k[i.toLowerCase()] || 1
  })
  if (
    Object.keys(k).length ===
    Object.entries(k).filter(([key, value]) => value >= 2).length
  ) {
    // console.log("string has repeats on each character")
    return ""
  }

  // Object.entries(k).forEach((key, value) => console.log(value))

  // console.log(Object.entries(k))
  // re = new RegExp(
  //   Object.entries(k).filter(([key, val]) => val === 1)[0][0],
  //   "gi"
  // )

  // console.log(Object.entries(k).filter(([key, val]) => val === 1)[0][0])
  console.log(
    s.match(
      new RegExp(
        Object.entries(k).filter(([key, val]) => val === 1)[0][0],
        "gi"
      )
    )[0]
  )
  // console.log(m)

  // console.log(Object.fromEntries(Object.entries(k).filter(([key, val]) => val === 1 )))
  // return Object.entries(k).filter(([key, val]) => val === 1 )[0][0]
  return s.match(
    new RegExp(Object.entries(k).filter(([key, val]) => val === 1)[0][0], "gi")
  )[0]
}

// firstNonRepeatingLetter('ssttsrrsaabbcc')
firstNonRepeatingLetter("sTreSS")
// firstNonRepeatingLetter('a')
