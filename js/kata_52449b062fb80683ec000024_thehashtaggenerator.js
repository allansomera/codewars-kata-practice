function generateHashtag(str) {
  if(str === "") return false
  let rep_space = str.replace(/ +/g, ' ')
  if(rep_space === " ") return false
  let arr = str.split(" ")
  let new_arr = arr.map(
    (i) => i.charAt(0).toUpperCase().replace(/^/, "#") + i.slice(1)
  )
  console.log(new_arr)
}

generateHashtag("Do We have A Hashtag")
generateHashtag(" ".repeat(20))
