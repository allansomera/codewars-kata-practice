function int32ToIp(int32) {
  // let x = (int32 >>> 0)
  return (int32 >>> 0)
    .toString(2)
    .padStart(32, "0")
    .match(/.{1,8}/g)
    .map((i) => parseInt(i, 2).toString())
    .join(".")
  // return (int32 >>> 0).toString(2).padStart(32, "0")
  // return x
}

console.log(int32toIp(2149583361))
// console.log(int32toIp(8))

// solutions:
// const int32ToIp = int32 => [24, 16, 8, 0].map(e => int32 >> e & 255).join('.');
//
// const int32ToIp = int32 => [24, 16, 8, 0].map(e => int32 >> e & 255).join`.`;
//
// function int32ToIp(int32){
//   return [int32 >>> 24, int32 >> 16 & 255, int32 >> 8 & 255, int32 & 255].join('.');
// }
