class User {
  constructor() {
    this.rank = -8
    this.progress = 0
  }
  incProgress() {
    console.log(`${this.progress}`)
  }
  getRank() {
    console.log(`${this.rank}`)
  }
}

let user = new User()
user.getRank()
