import { JSX } from "react";
import { TuonoRouteProps } from "tuono";

interface SessionPageProps {
    session_id: string
}

export default function SessionPage({
    data,
}: TuonoRouteProps<SessionPageProps>): JSX.Element | null {
    if (!data) return null;

    return (
        <h1 className="text-6xl font-bold">{data.session_id}</h1>
    )
}
