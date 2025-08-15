import { JSX } from "react";
import { TuonoRouteProps } from "tuono";
import { SessionPageProps } from "tuono/types";
import RandomSign from "../../components/RandomSign";

export default function SessionPage({
  data,
}: TuonoRouteProps<SessionPageProps>): JSX.Element | null {
  if (!data) return null;

  var session = data.session;

  return (
    <>
      <h1 className="text-6xl font-bold mb-6">Learning session</h1>
      <ul>
        <li>id: {session.id}</li>
        <li>created: {session.started_at}</li>
        <li>status: {session.status}</li>
        <li>completed_signs: {session.completed_signs}</li>
      </ul>

      <RandomSign session={session} />
    </>
  )
}
