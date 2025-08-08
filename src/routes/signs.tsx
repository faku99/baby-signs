import type { JSX } from 'react'
import { TuonoRouteProps } from 'tuono'

interface Sign {
  name: string
}

interface SignsProps {
  results: Array<Sign>
}

export default function SignsPage({
  data,
}: TuonoRouteProps<SignsProps>): JSX.Element | null {
  if (!data?.results) return null;

  return (
    <>
      <ul>
        {data.results.map((sign) => (
          <li key={sign.name}>
            {sign.name}
          </li>
        ))}
      </ul>
    </>
  )
}
