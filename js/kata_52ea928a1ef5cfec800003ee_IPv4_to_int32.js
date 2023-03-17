const ipToInt32 = (ip) => {
  // console.log(ip.split("."))
  // let x = ip
  return parseInt(
    ip
      .split(".")
      .map((i) => parseInt(i))
      .map((i) => (i >>> 0).toString(2).padStart(8, "0"))
      .join(""),
    2
  )
  // .split("")
  // .reverse()
  // .reduce(
  //   (ac, v, i, arr) => ac + (v ? Math.pow(2, arr.length - i - 1) : 0),
  //   0
  // )
  // console.log(parseInt(x, 2))
  // console.log(x)
}

let x = ipToInt32("128.32.10.1")
console.log(x)

// solutions:
// let ipToInt32 = ip => ip.split(".").reduce((a, b) => a << 8 | b) >>> 0;
//
// function ipToInt32(ip){
//    return ip.split(".").reduce(function(int,v){ return int*256 + +v } )
// }
//
// const ipToInt32 = ip =>
//   ip.split(`.`).reduce((pre, val) => 2 ** 8 * pre + +val);
//
// function ipToInt32(ip){
//   return ip.split(".").reduce(function(a, b) { return (a << 8) + parseInt(b, 10); }, 0) >>> 0;
// }
//
// function ipToInt32(ipv4Address) {
//   return ipv4Address.split`.`.reduce((int32, octet) => int32 << 8 | octet) >>> 0;
// }
