"use client";

import dynamic from "next/dynamic";

const Playground = dynamic(
  () => import("@/components/playground").then((m) => m.Playground),
  { ssr: false }
);

export default function Page() {
  return <Playground />;
}
