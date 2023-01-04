function generateHashtag(str) {
  let arr = str.split(" ")
  let new_arr = arr.map(
    (i) => i.charAt(0).toUpperCase().replace(/^/, "#") + i.slice(1)
  )
  console.log(new_arr)
}

generateHashtag("Do We have A Hashtag")
