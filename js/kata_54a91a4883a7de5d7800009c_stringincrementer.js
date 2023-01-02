const check09inc = (s) => {
  let re = /(9{1,})$/
  let num = 0
  if (s[0] === "0") {
    let m = s.match(re)
    //lenght of whole string is the greater than 9s matched string
    if (s.length > m[1].length) {
      num = +m[1]
      return ++num
    }
  } else {
    num = +s
    return ++num
  }
  // console.log(s.match(re))
}
function incrementString(strng) {
  const re = /^(\w+?(?:\d.*?)?\w+?)?([0-9]+)?$/
  const res = strng.match(re)
  const [whole, first, num, ...rest] = res
  console.log("check09", check09inc(num))
  // console.log(res)
  console.log(whole)
  // console.log(word)
  console.log(num)
}
incrementString("foobar99")
