function firstNonRepeatingLetter(s) {
    if (s === "") return 
    let k = {}
    s.split("").forEach(i => k[i] = ++k[i] || 1)
    if(Object.keys(k).length === Object.entries(k).filter(( [key, value] ) => value >=2 ).length){
        // console.log("string has repeats on each character")
        return ""
    }
    
    // console.log(k)
    console.log(Object.entries(k).filter(([key, val]) => val === 1 )[0][0])
    // console.log(Object.fromEntries(Object.entries(k).filter(([key, val]) => val === 1 )))
    // return Object.entries(k).filter(([key, val]) => val === 1 )[0][0]
}


// firstNonRepeatingLetter('ssttsrrsaabbcc')
firstNonRepeatingLetter('sTreSS')
// firstNonRepeatingLetter('a')