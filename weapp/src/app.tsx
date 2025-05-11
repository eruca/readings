import { PropsWithChildren } from 'react';

import { LiteratureProvider } from './context/LiteratureContext'
import './app.less'

function App({ children }: PropsWithChildren) {
  return (
    <LiteratureProvider>
      {children}
    </LiteratureProvider>
  )
}

export default App
