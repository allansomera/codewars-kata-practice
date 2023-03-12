function arrayDiff(a, b) {
  let dummy = [...a]
  if (b.length === 0) return a
  b.forEach((i) => {
    dummy = dummy.filter((d) => d !== i)
    console.log(dummy)
  })
  //console.log("dummy", dummy)
  return dummy
}
