import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'

import { Robot } from './components/Robot'

import './index.css'

createRoot(document.getElementById('root') as HTMLElement).render(
  <StrictMode>
    <Robot />
  </StrictMode>
)
