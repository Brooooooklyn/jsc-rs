setTimeout(() => {
  console.log('hello')
})

setTimeout(() => {
  console.log('hello after 1s')
}, 1000)

const i = setTimeout(() => {
  console.log('hello after 1s should be cancelled')
}, 1000)

clearTimeout(i)
