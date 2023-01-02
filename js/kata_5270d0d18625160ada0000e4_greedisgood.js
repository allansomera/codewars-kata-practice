const data_score = {
  1: 1000,
  2: 200,
  3: 300,
  4: 400,
  5: 500,
  6: 600,
}
const score = (dice) => {
  const obj = {}
  const leftover_1_5 = {}
  dice.forEach((i) => (obj[i] = ++obj[i] || 1))
  console.log("obj", obj)
  let sum = 0
  for (const p in obj) {
    // console.log('p is typeof ', typeof p)
    if (obj[p] >= 3) {
      sum += data_score[p]
      if (p === "1") {
        leftover_1_5[p] = obj[p] % 3
      }
      if (p === "5") {
        leftover_1_5[p] = obj[p] % 3
      }
      console.log("after 3", sum)
    } else {
      if (p === "1") sum +=  (obj[p] * 100)
      else if (p === "5") sum += (obj[p] * 50)
      else sum += 0
    }
  }
  if (Object.keys(leftover_1_5) !== 0) {
    for (const l in leftover_1_5) {
      if (l === "1") sum += leftover_1_5[l] * 100
      if (l === "5") sum += leftover_1_5[l] * 50
    }
  }
  console.log("leftover", leftover_1_5)
  console.log(sum)
  return sum
}
// score([2, 4, 4, 5, 4])
// score([1, 1, 1, 1, 1])
// score([1, 1, 1, 1, 3])
score([1, 5, 1, 3, 4])
