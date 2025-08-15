import { JSX } from "react";
import { Session } from "tuono/types";

interface RandomSignProps {
  session: Session,
}

export default function RandomSign({
  session,
}: RandomSignProps): JSX.Element | null {
  return (
    <>
      <h1 className="mt-16">Random Sign ({session.id})</h1>
    </>
  )
}
