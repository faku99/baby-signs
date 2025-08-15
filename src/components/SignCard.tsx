import { JSX } from "react";
import { Sign } from "tuono/types";

interface SignCardProps {
  sign: Sign,
}

export default function SignCard({
  sign,
}: SignCardProps): JSX.Element {
  return (
    <div className="bg-gray-500 rounded h-32">
      <h1 className="text-3xl font-bold text-center text-white pt-12">
        {sign.id}
      </h1>
    </div>
  )
}
