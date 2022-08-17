import { renderToReadableStream } from 'react-dom/server'

function App() {
  return <div>Hello World</div>
}

const readableStream = await renderToReadableStream(<App />)

console.info(readableStream.allReady)
