import type { JSX } from 'react'
import { TuonoScripts } from 'tuono'
import type { TuonoLayoutProps } from 'tuono'

import '../styles/global.css'
import NavBar from '../components/NavBar'

export default function RootLayout({
  children,
}: TuonoLayoutProps): JSX.Element {
  return (
    <html>
      <body>
        <NavBar />
        <main>{children}</main>
        <TuonoScripts />
      </body>
    </html>
  )
}
