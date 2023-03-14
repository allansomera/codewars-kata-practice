function duplicateCount(text) {
  let split_text = text.split("")
  let split_obj = {}
  split_text.forEach((i) => {
    x = i.toLowerCase()
    split_obj[x] = ++split_obj[x] || 1
  })

  // split_count = Object.keys(split_obj).filter((x) => split_obj[x] >= 2)
  // return Object.keys(split_obj).filter((x) => split_obj[x] >= 2).length

  // console.log(split_count.length)
  console.log(split_obj)
}

// duplicateCount("aaaaabcdefg1111234")
// duplicateCount("123")
// duplicateCount("aabBcde")

// good solutions
// function duplicateCount(text){
//   return (text.toLowerCase().split('').sort().join('').match(/([^])\1+/g) || []).length;
// }
//
// function duplicateCount(text){
//   return new Set(text.toLowerCase().match(/(.)(?=.*\1)/gi)).size
// }
//
// function duplicateCount(text){
//   var count = text.toLowerCase().split('').reduce((accum, curr) => {
//     accum[curr] ? accum[curr] += 1 : accum[curr] = 1;
//     return accum;
//   }, {});
//   return Object.keys(count).filter(key => count[key] > 1).length;
// }
