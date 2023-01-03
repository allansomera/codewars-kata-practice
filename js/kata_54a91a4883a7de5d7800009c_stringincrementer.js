const check09inc = (s) => {
  if (s === undefined) return 1
  let re_0 = /^(.*0{1,})/
  let re_9 = /(9{1,})$/
  let num = 0
  // let m9 = s.match(re_9)
  // console.log("re_0 match", m0)
  // console.log("re_9 match", m9)
  // console.log("re_0 test", re_0.test(s))
  // console.log("re_9 test", re_9.test(s))
  // if zeroes exist in string
  if (re_0.test(s)) {
    let m0 = s.match(re_0)
    console.log("zeroes exist: ", m0)
    let z_len = m0[1].length
    let m9 = s.match(re_9)
    if (re_9.test(s)) {
      console.log("9s exist")
      let m9 = s.match(re_9)
      //check if the 9s is less than the length of the string including the zeroes
      num = +m9[1]
      ++num
      let num_len = num.toString().length
      //when inc, and a new digit position is added
      if (num_len > m9[1].length) {
        let new_0_str = s.slice(0, m0[1].length - 1)
        return new_0_str + num
      }
    } else {
      //check if right most digit is a zero
      if (s.charAt(s.length - 1) === "0") {
        let new_0_str = s.slice(0, m0[1].length - 1)
        num = +s.charAt(s.length - 1)
        console.log("right most zero", new_0_str)
        console.log("added increment", new_0_str + (num + 1))
        return new_0_str + ++num
      } else {
        // let new_0_str = s.slice(0, m0[1].length)
        // console.log("new_0_str", new_0_str)
        console.log("there are no 9s: ", m0[1])
        num = +s.charAt(s.length - 1)
        console.log("right most digit not zero", m0[1] + (num + 1))
        // console.log(num)
        return m0[1] + ++num
      }
      // console.log("final", new_0_str + (++num))
      console.log("inside check09inc")
    }
  } else {
    num = +s
    return ++num
  }
}
function incrementString(strng) {
  const re = /^(\w+?(?:\d.*?)?\w+?)?([0-9]+)?$/
  const res = strng.match(re)
  let [whole, first, num, ...rest] = res
  // console.log("inc", check09inc(num))
  console.log(res)
  console.log(whole)
  console.log(first)
  console.log(num)
  // return first + check09inc(num)
  if (first === undefined && num) {
    let n = ++num
    console.log("n is", n)
  } else if (first === undefined) {
    console.log("first undefined")
    console.log("" + check09inc(num))
  } else if (first && num === undefined) {
    if (first.charAt(0) !== "0") {
      return first + 1
    } else {
      console.log(check09inc(first))
    }
  }
  console.log(first + check09inc(num))
}
incrementString("009")
