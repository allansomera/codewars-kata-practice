function mostFrequentItemCount(collection) {
  if (collection.length === 0) return 0
  let count_obj = {}
  collection.forEach((i) => {
    count_obj[i] = ++count_obj[i] || 1
  })
  // const sorted = Object.keys(count_obj).sort((a, b) => {
  //   return count_obj[b] - count_obj[a]
  // })
  // .reduce((prev, curr, i) => {
  //   prev[i] = count_obj[curr]
  //   return prev
  // }, {})
  return count_obj[
    Object.keys(count_obj).sort((a, b) => {
      return count_obj[b] - count_obj[a]
    })[0]
  ]
  // return count_obj[sorted[0]]
}

console.log(mostFrequentItemCount([3, -1, -1]))

// nice solution:
// const mostFrequentItemCount = (collection) =>
// 	Math.max(
// 		...Object.values(
// 			collection.reduce(
// 				(acc, el) => {
// 					acc[el] = ++acc[el] || 1;
// 					return acc;
// 				},
// 				{ 0: 0 }
// 			)
// 		)
// 	);
