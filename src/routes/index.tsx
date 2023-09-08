import { api } from "../main";

export default function Index() {
  const { data: version } = api.useQuery(["version"]);

  return (
    <div className="flex h-screen flex-col items-center justify-center gap-y-4">
      <h1 className="text-5xl font-extrabold tracking-tighter">
        Tauri + rspc + Prisma
      </h1>
      <br />
      <p>
        Version: {version}
      </p>
    </div>
  );
}
