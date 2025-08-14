import { JSX } from "react";
import { Sign } from "tuono/types";

interface SignCardProps {
  sign: Sign,
}

export default function SignCard({
  sign,
}: SignCardProps): JSX.Element {
  return (
    <div className="h-32 w-32 bg-gray-500 rounded">
      <h1 className="text-3xl font-bold text-center text-white pt-12">
        {sign.id}
      </h1>
    </div>
  )
}
