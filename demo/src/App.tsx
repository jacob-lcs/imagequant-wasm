import React, { useEffect } from 'react'
import { greet } from 'imagequant-wasm'

const App = () => {

  useEffect(() => {
    greet()
  }, [])

  return (
    <div>11</div>
  )
}

export default App
