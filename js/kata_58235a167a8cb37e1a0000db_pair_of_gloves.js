function numberOfPairs(gloves) {
  // let set_item = new Set()
  // console.log(set_pair)
  const item_count = {}
  let sum = 0
  gloves.forEach((i) => {
    // set_item.add(i)
    item_count[i] = ++item_count[i] || 1
  })
  Object.keys(item_count).forEach((key) => {
    if (item_count[key] >= 2) sum += Math.floor(item_count[key] / 2)
  })

  // for (const i of set_item) item_count[i] = ++item_count[i] || 1

  console.log(item_count)
  return sum
}

// console.log(
//   numberOfPairs(["gray", "black", "purple", "purple", "gray", "black"])
// )

console.log(
  numberOfPairs([
    "Teal",
    "Aqua",
    "Fuchsia",
    "Teal",
    "Silver",
    "Olive",
    "Teal",
    "Green",
    "Maroon",
    "Green",
    "Maroon",
    "White",
    "Fuchsia",
    "White",
    "White",
    "Maroon",
    "Blue",
    "Lime",
    "Yellow",
    "Yellow",
    "Black",
    "Blue",
    "Silver",
    "Fuchsia",
    "White",
    "Silver",
    "Lime",
    "Olive",
    "Gray",
    "Teal",
    "Gray",
    "Green",
    "White",
    "Black",
    "Lime",
    "Fuchsia",
    "Purple",
    "Black",
    "Olive",
    "Gray",
    "Silver",
    "White",
    "Olive",
    "Blue",
    "Red",
    "Gray",
    "Purple",
    "Purple",
    "Black",
    "Gray",
    "White",
    "Yellow",
    "Aqua",
    "Green",
    "Fuchsia",
    "Blue",
    "Teal",
    "Blue",
    "Blue",
    "Navy",
    "Silver",
    "Aqua",
    "Purple",
    "Purple",
    "Yellow",
    "Red",
    "Green",
    "Fuchsia",
    "Green",
    "Fuchsia",
    "Yellow",
    "Teal",
    "Olive",
    "White",
    "Yellow",
    "Gray",
    "Yellow",
    "Olive",
    "Blue",
    "Fuchsia",
    "Yellow",
    "Gray",
    "Purple",
    "Green",
    "Navy",
    "White",
    "Lime",
    "Green",
    "Black",
    "Maroon",
    "Maroon",
    "White",
    "White",
    "Gray",
    "White",
    "Teal",
    "White",
    "Blue",
    "Maroon",
  ])
)
