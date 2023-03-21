const nato = {
  a: "alfa",
  b: "bravo",
  c: "charlie",
  d: "delta",
  e: "echo",
  f: "foxtrot",
  g: "golf",
  h: "hotel",
  i: "india",
  j: "juliett",
  k: "kilo",
  l: "lima",
  m: "mike",
  n: "november",
  o: "oscar",
  p: "papa",
  q: "quebec",
  r: "romeo",
  s: "sierra",
  t: "tango",
  u: "uniform",
  v: "victor",
  w: "whiskey",
  x: "xray",
  y: "yankee",
  z: "zulu",
}
function to_nato(words) {
  let message = []
  words
    .split("")
    .filter((i) => i.match(/[^\s]/g))
    .map((i) => i.toLowerCase())
    .forEach((i) => {
      i.match(/[,.!?\s]/i)
        ? message.push(i)
        : message.push(nato[i].charAt(0).toUpperCase() + nato[i].slice(1))
    })
  console.log(message)
  // return message.filter((i) => i.match(/[^\s]/g)).join(" ")
  return message.join(" ")
}

let t = to_nato("If you can, read")
console.log(t)

// solutions:
// const to_nato = words => words
//   .toLowerCase()
//   .replace(/\s+/g, '')
//   .split('')
//   .map(char => NATO.hasOwnProperty(char) ? NATO[char] : char)
//   .join(' ');
//
// const to_nato = words =>
//   (obj => words.toLowerCase().replace(/./g, val => obj[val] || ``).trim())
//   ({a:`Alfa `, b:`Bravo `, c:`Charlie `, d:`Delta `, e:`Echo `, f:`Foxtrot `, g:`Golf `, h:`Hotel `, i:`India `,
//     j:`Juliett `, k:`Kilo `, l:`Lima `, m:`Mike `, n:`November `, o:`Oscar `, p:`Papa `, q:`Quebec `, r:`Romeo `,
//     s:`Sierra `, t:`Tango `, u:`Uniform `, v:`Victor `, w:`Whiskey `, x:`Xray `, y:`Yankee `, z:`Zulu `,
//     '.': `. `, '!':`! `, '?':`? `});
//
// const to_nato = s => s
//   .replace(/[a-z]/gi, x => ` ${NATO[x.toLowerCase()]} `)
//   .replace(/\s+/g, ' ')
//   .trim();
//
// const to_nato = (words) =>
//   words
//     .toLowerCase()
//     .replace(/[^a-z,.!?]/g, '')
//     .replace(/[a-z,.!?]/g, x => `${NATO[x] || x} `)
//     .trim();
//
// b={a:'Alfa',b:'Bravo',c:'Charlie',d:'Delta',e:'Echo',f:'Foxtrot',g:'Golf',h:'Hotel',i:'India',j:'Juliett',k:'Kilo',l:'Lima',m:'Mike',n:'November',o:'Oscar',p:'Papa',q:'Quebec',r:'Romeo',s:'Sierra',t:'Tango',u:'Uniform',v:'Victor',w:'Whiskey',x:'Xray',y:'Yankee',z:'Zulu'};
// to_nato=a=>a.toLowerCase().replace(/ /g,'').split``.map(a=>b[a]||a).join` `;
//
// note:
// .map(a=>b[a] || a) //if b[a] does not exist, use a
