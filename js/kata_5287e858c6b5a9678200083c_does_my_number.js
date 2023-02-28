function narcissistic(value) {
  // Code me to return true or false
  let value_string = value.toString()
  let sum = 0

  for (let i = 0; i < value_string.length; i++) {
    // num_array.push(parseInt(value_string[i]) ** value_string.length)
    sum += parseInt(value_string[i]) ** value_string.length
  }
  // num_array = num_array.map((i) => i ** num_array.length)
  return sum === value ? true : false
}

console.log(narcissistic(153))
