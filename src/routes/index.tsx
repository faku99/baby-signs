import type { JSX } from 'react'
import { Link } from 'tuono'

export default function IndexPage(): JSX.Element {
  return (
    <>
      <h1 className='text-6xl font-bold text-center'>
        Baby Signs
      </h1>
      <ul className='list-disc'>
        <li><Link href="/signs">Signs list</Link></li>
        <li><a href="/session/new">Start a new session</a></li>
      </ul>
    </>
  )
}
