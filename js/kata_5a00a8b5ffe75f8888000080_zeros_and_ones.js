const replaceZero = (arr) => {
  // console.log(arr)
  let idx_arr = []
  let zero = 0
  let idx = arr.indexOf(zero)
  for (; idx !== -1; ) {
    idx_arr.push(idx)
    idx = arr.indexOf(zero, idx + 1)
  }
  // console.log(`idx_arr: ${idx_arr}`)
  let temp = [...arr]
  let count_obj = {}

  // console.log(`temp copy: ${temp}`)
  for (let i = 0; i < idx_arr.length; i++) {
    // console.log(`idx_arr[${i}]: ${idx_arr[i]}`)
    temp[idx_arr[i]] = 1
    // console.log(`temp is: ${temp}`)
    let s = temp.toString().split(",").join("").split("0")
    // console.log(`s is: ${s}`)
    let long = s.reduce((a, b) => (a.length > b.length ? a : b), "")
    // console.log(`long is: ${long}`)
    temp[idx_arr[i]] = 0
    count_obj[idx_arr[i]] = long.length
  }
  console.log(count_obj)
  // console.log(
  //   Object.keys(count_obj).reduce((a, b) =>
  //     count_obj[a] > count_obj[b] ? parseInt(a) : parseInt(b)
  //   )
  // )
  return parseInt(
    Object.keys(count_obj).reduce((a, b) =>
      count_obj[a] > count_obj[b] ? parseInt(a) : parseInt(b)
    )
  )
  // return Object.keys(count_obj).reduce((a, b) =>
  //   count_obj[a] > count_obj[b] ? parseInt(a) : parseInt(b)
  // )
}

// console.log(
//   replaceZero([1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1])
// )
console.log(replaceZero([1, 1, 0, 1, 1, 0, 1, 1]))
