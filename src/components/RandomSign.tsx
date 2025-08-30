import { JSX, useEffect, useState } from "react";
import { Session, Sign } from "tuono/types";

export default function RandomSign(): JSX.Element | null {
  const [sign, setSign] = useState<Sign | null>(null);

  const fetchSign = async () => {
    try {
      const response = await fetch('/api/session');
      const data: Sign = await response.json();
      setSign(data);
    } catch (e) {
      console.error(`Failed to fetch: ${e}`);
    } finally {
      // setLoading(false);
    }
  };

  useEffect(() => {
    fetchSign();
  }, []);

  if (!sign) {
    return (
      <div className="mt-16">Loading...</div>
    );
  }

  return (
    <div className="mt-16">Sign: {sign.id}</div>
  );
}
