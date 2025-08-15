import type { JSX } from 'react';
import { TuonoRouteProps } from 'tuono';
import { Sign } from 'tuono/types';
import SignCard from '../components/SignCard';

interface SignsProps {
  results: Array<Sign>
}

export default function SignsPage({
  data,
}: TuonoRouteProps<SignsProps>): JSX.Element | null {
  if (!data?.results) return null;

  return (
    <>
      { /* TODO: Calculate grid's columns based on cards' width */ }
      <div className="grid grid-cols-4 gap-4 items-stretch">
        {data.results.map((sign) => (
          <SignCard sign={sign} />
        ))}
      </div>
    </>
  )
}
