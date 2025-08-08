import { JSX } from "react";
import { Link } from "tuono";

export default function NavBar(): JSX.Element {
  return (
    <nav className="h-12 bg-navbar-background text-navbar-foreground w-full flex flex-row items-center mb-4">
      <div className="flex flex-row items-center w-1/2 mx-auto">
        <h1 className="text-3xl"><Link href="/">Baby Signs</Link></h1>
      </div>
    </nav>
  )
}
