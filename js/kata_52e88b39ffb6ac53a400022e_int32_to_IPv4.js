function int32toIp(int32) {
  return (int32 >>> 0).toString(2)
}

console.log(int32toIp(9))
