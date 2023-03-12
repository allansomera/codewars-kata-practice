function woofDecoder(str) {
  // Woof-woof!
  let woof = "woof"
  // let s_pattern = woof.split("").join(".*")
  // let pattern = new RegExp(s_pattern, "gi")
  let pattern = /woof/gi
  let pattern2 = /w.*o.*.o.*f/g
  let alpha_string = "abcdefghijklmnopqrstuvwxyz"
  let split_arr = []

  if (str.includes("!")) {
    split_arr = [...str.split("!")].filter((i) => i)
    let message = []
    if (split_arr.length > 0) {
      // console.log(split_arr)
      let count = 0
      split_arr.forEach((i) => {
        if (i.includes("-")) {
          split_dash = i.split("-")
          let count = 0
          split_dash.forEach((x) => {
            if (x.match(pattern).length >= 1) {
              console.log(x.match(pattern))
              count++
            }
            // else {
            //   count++
            //   console.log(`x is: ${x}`)
            // }
            // count++
          })
          message.push(alpha_string[count - 1])
        }
        // if (i.match(pattern) !== null) {
        //   let len = i.match(pattern).length
        //   message.push(alpha_string[i.match(pattern).length - 1])
        // } else if (i.match(pattern2) !== null) {
        //   console.log(`pattern2: ${i.match(pattern2)}`)
        // }
        // console.log(alpha_string[i.match(pattern).length - 1])
        // console.log(i.match(pattern))
      })
      console.log(message.toString().split(",").join(""))
      // else {
      //   if (split_arr[0].match(pattern2) !== null) {
      //     split_arr.forEach((i) => {
      //       console.log(i.match(pattern2))
      //     })
      //   }
      // }
    }
    // else if (split_arr.length === 0) {
    //   console.log("use pattern2")
    // }
  }

  // console.log((str.match(pattern) || []).length)
  // console.log(str.match(pattern))
  // console.log(split_arr)
}

// woofDecoder(
//   "aWoof-woof-woof-woof-woof-woof-woof!woof woof woof! woof woof woof woof woof!"
// )
woofDecoder("Woof! Woof!" + "-woof".repeat(26) + "!")
// woofDecoder(
//   "Woofshhhwoof-woof-woof-woof-woof-woof!Woof-woof-woof-woof-woof! " +
//     "Woof-woof-woof-woof-woof! Woof-woof-woof-woof! " +
//     "Woof-woof-woof-woof-woof-woof-woof-woof-woof-woof-woof-woof-woof! " +
//     "Woof-woof-woof-woof-woof!"
// )
// woofDecoder("awoof")
// woofDecoder("awereroereroerefer!")
